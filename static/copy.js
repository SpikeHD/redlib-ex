// @license http://www.gnu.org/licenses/agpl-3.0.html AGPL-3.0
async function copy() {
    await navigator.clipboard.writeText(document.getElementById('bincode_str').value);
}

async function copy_comment_permalink(event) {
    event.preventDefault();
    const link = event.currentTarget;
    await navigator.clipboard.writeText(new URL(link.dataset.url, window.location.origin).toString());
}

function set_listener() {
    const copy_button = document.getElementById('copy');
    if (copy_button) {
        copy_button.addEventListener('click', copy);
    }

    document.querySelectorAll('.comment_permalink').forEach((el) => {
        el.addEventListener('click', copy_comment_permalink);
    });
}

window.addEventListener('load', set_listener);
// @license-end