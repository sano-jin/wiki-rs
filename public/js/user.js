window.addEventListener("DOMContentLoaded", () => {
  // 新しいユーザを作る
  // (1) 送信ボタンのHTMLを取得
  const create_user_submit = document.getElementById("create_user_submit");
  create_user_submit.addEventListener("click", async (e) => {
    e.preventDefault();

    // (3) フォーム入力欄のHTMLを取得
    const create_user_name = document.querySelector(
      "input[name=create_user_name]"
    );
    const name = create_user_name.value;
    if (name.length === 0) return;

    // (3) フォーム入力欄のHTMLを取得
    const create_user_password = document.querySelector(
      "input[name=create_user_password]"
    );
    const password = create_user_password.value;
    if (password.length === 0) return;

    console.log("uploading", name, password);

    const formData = new FormData();
    formData.append("name", name);
    formData.append("password", password);
    console.log("sending", formData);

    const body = JSON.stringify({
      name: name,
      password: password,
    }); // This is your file object
    console.log("sending", body);

    await fetch("/user", {
      // Your POST endpoint
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        // 'Content-Type': 'application/x-www-form-urlencoded',
      },
      body: body,
    });

    const location = "/users";
    window.location = location;
  });
  //

  // ユーザを削除
  const delete_user_submit = document.getElementById("delete_user_submit");
  delete_user_submit.addEventListener("click", async (e) => {
    e.preventDefault();

    // (3) フォーム入力欄のHTMLを取得
    const delete_user_name = document.querySelector(
      "input[name=delete_user_name]"
    );
    const name = delete_user_name.value;
    if (name.length === 0) return;

    // const location = "/user?name=" + encodeURIComponent(path);
    // window.location = location;
    const path = encodeURIComponent(name);

    await fetch("/user?" + new URLSearchParams({ name: name }), {
      method: "DELETE",
    });

    const location = "/users";
    window.location = location;
  });

  //

  //
});
