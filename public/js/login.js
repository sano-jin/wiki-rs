"use strict";

/** login
 */
window.addEventListener("DOMContentLoaded", () => {
  // (1) 送信ボタンのHTMLを取得
  const login_user_submit = document.getElementById("login_user_submit");
  login_user_submit.addEventListener("click", async (e) => {
    e.preventDefault();

    // (3) フォーム入力欄のHTMLを取得
    const login_user_name = document.querySelector(
      "input[name=login_user_name]"
    );
    const name = login_user_name.value;
    if (name.length === 0) return;

    // (3) フォーム入力欄のHTMLを取得
    const login_user_password = document.querySelector(
      "input[name=login_user_password]"
    );
    const password = login_user_password.value;
    if (password.length === 0) return;

    console.log("uploading", name, password);

    const body = JSON.stringify({
      name: name,
      password: password,
    });

    console.log("sending", body);

    fetch("/login", {
      // Your POST endpoint
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: body,
    })
      .then((response) => {
        if (!response.ok) {
          console.error("response.ok:", response.ok);
          console.error("esponse.status:", response.status);
          console.error("esponse.statusText:", response.statusText);
          throw new Error(response.statusText);
        }

        // ここに成功時の処理を記述
        console.log(response);
        if (response.status === 200) {
          console.log("成功！");

          // const location = "/users";
          // window.location = location;
        } else {
          console.log("認証に失敗しました！");
          window.alert("Authorization failed");
        }
      })
      .catch((error) => {
        // ネットワークエラーでも !response.ok でもここで処理できる
        console.error("エラーが発生しました", error);
      });
  });
});
