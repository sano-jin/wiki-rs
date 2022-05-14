window.addEventListener("DOMContentLoaded", () => {
  // 送信ボタンのHTMLを取得
  const btn_submit = document.getElementById("btn_submit");

  btn_submit.addEventListener("click", async (e) => {
    e.preventDefault();

    // (3) フォーム入力欄のHTMLを取得
    const input_path = document.querySelector("input[name=input_path]");
    const path = input_path.value;

    // (3) フォーム入力欄のHTMLを取得
    const input_content = document.querySelector(
      "textarea[name=input_content]"
    );

    // (4) FormDataオブジェクトにデータをセット
    const body = input_content.value;

    // (5) フォームの入力値を送信
    const response = await fetch("/edit", {
      method: "POST", // *GET, POST, PUT, DELETE, etc.
      mode: "cors", // no-cors, *cors, same-origin
      cache: "no-cache", // *default, no-cache, reload, force-cache, only-if-cached
      credentials: "same-origin", // include, *same-origin, omit
      headers: {
        "Content-Type": "application/json",
        // 'Content-Type': 'application/x-www-form-urlencoded',
      },
      redirect: "follow", // manual, *follow, error
      referrerPolicy: "no-referrer", // no-referrer, *no-referrer-when-downgrade, origin, origin-when-cross-origin, same-origin, strict-origin, strict-origin-when-cross-origin, unsafe-url
      body: JSON.stringify({ path: path, body: body }), // body data type must match "Content-Type" header
    });

    const location = await response.json();
    console.log("location: ", location);

    // redirect to the returned location
    window.location = location;
  });
});
