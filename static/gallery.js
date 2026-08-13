function initGallery(container) {
	const items = container.querySelectorAll(".gallery_item");
	const prev = container.querySelector("[data-gallery-prev]");
	const next = container.querySelector("[data-gallery-next]");
	const counter = container.querySelector("[data-gallery-counter]");

	if (items.length === 0) {
		return;
	}

	let index = 0;

	function show(i) {
		index = (i + items.length) % items.length;
		items.forEach((item, n) => {
			item.style.display = n === index ? "" : "none";
		});
		if (counter) {
			counter.textContent = `${index + 1} / ${items.length}`;
		}
	}

	if (prev) {
		prev.addEventListener("click", () => show(index - 1));
	}
	if (next) {
		next.addEventListener("click", () => show(index + 1));
	}
	show(0);
}

document.querySelectorAll("[data-gallery]").forEach(initGallery);