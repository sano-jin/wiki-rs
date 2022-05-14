window.addEventListener("DOMContentLoaded", () => {
  // 新しいページを作る
  // (1) 送信ボタンのHTMLを取得
  const btn_submit = document.getElementById("btn_submit");
  btn_submit.addEventListener("click", async (e) => {
    e.preventDefault();

    // (3) フォーム入力欄のHTMLを取得
    const input_path = document.querySelector("input[name=input_path]");
    const path = input_path.value;
    if (path.length === 0) return;

    const location = "/edit?path=" + encodeURIComponent(path);
    window.location = location;
  });
});
