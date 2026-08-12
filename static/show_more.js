// @license http://www.gnu.org/licenses/agpl-3.0.html AGPL-3.0
const CLAMP_HEIGHT = 250;

function initShowMore() {
    document.querySelectorAll('.post_body.post_preview_wrap').forEach(function (body) {
        const preview = body.querySelector('.post_preview');
        const btn = body.querySelector('.post_show_more');
        if (!preview || !btn) return;

        if (preview.scrollHeight > CLAMP_HEIGHT) {
            body.classList.add('clamped');
            btn.hidden = false;
        }
    });
}

document.addEventListener('click', function (e) {
    const btn = e.target.closest('.post_show_more');
    if (!btn) return;
    const body = btn.closest('.post_body');
    if (!body) return;

    const expanded = body.classList.toggle('clamped');
    btn.textContent = expanded ? 'Show More' : 'Show Less';
});

if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initShowMore);
} else {
    initShowMore();
}
// @license-end