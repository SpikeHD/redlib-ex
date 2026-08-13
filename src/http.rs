use hyper::client::HttpConnector;
use hyper::header::{CONTENT_TYPE, HeaderName, HeaderValue};
use hyper::{Body, Client as HyperClient, Method, Request, Response, Uri};
use hyper_tls::HttpsConnector;
use log::info;
use serde_json::Value;
use std::sync::LazyLock;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub static CLIENT: LazyLock<Client> = LazyLock::new(build_client);

#[derive(Clone)]
pub struct Client {
	inner: HyperClient<HttpsConnector<HttpConnector>, Body>,
}

pub fn build_client() -> Client {
	let https = HttpsConnector::new();
	let client = HyperClient::builder().build::<_, Body>(https);
	info!("Built hyper (hyper-tls) client with native-tls");
	Client { inner: client }
}

impl Client {
	pub fn get(&self, uri: Uri) -> RequestBuilder {
		self.request(Method::GET, uri)
	}

	pub fn post(&self, uri: Uri) -> RequestBuilder {
		self.request(Method::POST, uri)
	}

	pub fn request(&self, method: Method, uri: Uri) -> RequestBuilder {
		let request = Request::builder().method(method).uri(uri).body(Body::empty()).expect("Failed to build request");
		RequestBuilder {
			client: self.inner.clone(),
			request,
		}
	}
}

pub struct RequestBuilder {
	client: HyperClient<HttpsConnector<HttpConnector>, Body>,
	request: Request<Body>,
}

impl RequestBuilder {
	pub fn header<K, V>(mut self, key: K, value: V) -> Self
	where
		K: AsHeaderName,
		V: AsHeaderValue,
	{
		if let (Ok(name), Ok(val)) = (key.into_name(), value.into_value()) {
			self.request.headers_mut().insert(name, val);
		}
		self
	}

	pub fn json(mut self, value: &Value) -> Self {
		self.request.headers_mut().insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
		let body = Body::from(value.to_string());
		self.request = self.request.map(move |_| body);
		self
	}

	pub fn body<B: Into<Body>>(mut self, body: B) -> Self {
		let body = body.into();
		self.request = self.request.map(move |_| body);
		self
	}

	pub async fn send(self) -> Result<Response<Body>, Error> {
		Ok(self.client.request(self.request).await?)
	}
}

pub trait ResponseExt {
	fn json(self) -> impl std::future::Future<Output = Result<Value, Error>> + Send;
	fn text(self) -> impl std::future::Future<Output = Result<String, Error>> + Send;
	fn bytes(self) -> impl std::future::Future<Output = Result<Vec<u8>, Error>> + Send;
}

impl ResponseExt for Response<Body> {
	async fn json(self) -> Result<Value, Error> {
		let bytes = self.bytes().await?;
		Ok(serde_json::from_slice(&bytes)?)
	}

	async fn text(self) -> Result<String, Error> {
		let bytes = self.bytes().await?;
		Ok(String::from_utf8(bytes)?)
	}

	async fn bytes(self) -> Result<Vec<u8>, Error> {
		let bytes = hyper::body::to_bytes(self.into_body()).await?;
		Ok(bytes.as_ref().to_vec())
	}
}

pub trait AsHeaderName {
	fn into_name(self) -> Result<HeaderName, Error>;
}

impl AsHeaderName for &str {
	fn into_name(self) -> Result<HeaderName, Error> {
		Ok(HeaderName::from_bytes(self.as_bytes())?)
	}
}

impl AsHeaderName for String {
	fn into_name(self) -> Result<HeaderName, Error> {
		Ok(HeaderName::from_bytes(self.as_bytes())?)
	}
}

impl AsHeaderName for &String {
	fn into_name(self) -> Result<HeaderName, Error> {
		Ok(HeaderName::from_bytes(self.as_bytes())?)
	}
}

impl AsHeaderName for HeaderName {
	fn into_name(self) -> Result<HeaderName, Error> {
		Ok(self)
	}
}

pub trait AsHeaderValue {
	fn into_value(self) -> Result<HeaderValue, Error>;
}

impl AsHeaderValue for &str {
	fn into_value(self) -> Result<HeaderValue, Error> {
		Ok(HeaderValue::from_str(self)?)
	}
}

impl AsHeaderValue for String {
	fn into_value(self) -> Result<HeaderValue, Error> {
		Ok(HeaderValue::from_str(&self)?)
	}
}

impl AsHeaderValue for &String {
	fn into_value(self) -> Result<HeaderValue, Error> {
		Ok(HeaderValue::from_str(self)?)
	}
}

impl AsHeaderValue for &[u8] {
	fn into_value(self) -> Result<HeaderValue, Error> {
		Ok(HeaderValue::from_bytes(self)?)
	}
}
