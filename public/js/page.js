const copyToClipboard = (text) => {
  const textArea = document.createElement("textarea");
  textArea.value = text;

  // Avoid scrolling to bottom
  textArea.style.top = "0";
  textArea.style.left = "0";
  textArea.style.position = "fixed";

  document.body.appendChild(textArea);
  textArea.focus();
  textArea.select();

  document.execCommand("copy");

  textArea.parentNode.removeChild(textArea);
};

window.addEventListener("DOMContentLoaded", () => {
  // ページの削除
  const btn_submit = document.getElementById("btn-submit-delete");
  if (btn_submit) {
    btn_submit.addEventListener("click", async (e) => {
      e.preventDefault();

      // フォームの入力値を送信
      const response = await fetch(
        "/edit?" + new URLSearchParams({ path: __title__ }),
        { method: "DELETE" }
      );

      // redirect to the home page
      window.location = "/";
    });
  }

  // ファイルの添付
  const btn_select_attach = document.getElementById("btn-select-attach");
  // const btn_post_attach = document.getElementById("btn-post-attach");
  // Select your input type file and store it in a variable

  // This will upload the file after having read it
  const upload = async (file) => {
    const formData = new FormData();
    formData.append("filename", file);

    // await fetch("/attach?" + new URLSearchParams({ path: "{{ PATH }}" }), {
    await fetch("/attach?" + new URLSearchParams({ path: __title__ }), {
      // Your POST endpoint
      method: "POST",
      body: formData, // This is your file object
    });

    // reload the page
    location.reload();
  };

  // Event handler executed when a file is selected
  const onSelectFile = () => upload(btn_select_attach.files[0]);

  // Add a listener on your input
  // It will be triggered when a file will be selected
  btn_select_attach.addEventListener("change", onSelectFile, false);

  // ファイルの削除
  const btn_delete_attaches =
    document.getElementsByClassName("btn-delete-attach");
  for (let i = 0; i < btn_delete_attaches.length; i++) {
    const btn = btn_delete_attaches[i];
    const filename = btn.parentElement.firstChild.innerText;
    btn.addEventListener("click", async (e) => {
      console.log("deleting attach", filename);
      e.preventDefault();

      // const encodedFilename = encodeURIComponent(filename);

      const file = filename.split("\\").pop().split("/").pop();
      console.log("file: ", file, "path: ", __path__);

      console.log(__path__);
      // フォームの入力値を送信
      const response = await fetch(
        "/attach?" +
          new URLSearchParams({
            path: __title__,
            file: file,
          }),
        { method: "DELETE" }
      );

      console.log(response);

      // reload the page
      location.reload();
    });
  }

  // ソースコードをコピーできるボタンの追加
  // const btn_submit = document.getElementById("btn-submit-delete");
  const collection = document.getElementsByTagName("pre");
  for (let cell of collection) {
    // create a new div element
    const newDiv = document.createElement("div");
    newDiv.classList.add("code-copy-button");

    const newI = document.createElement("i");
    newI.classList.add("fa-solid");
    newI.classList.add("fa-copy");

    newDiv.appendChild(newI);

    newDiv.addEventListener("click", async (e) => {
      e.target.classList.add("copied");
      e.preventDefault();

      const code = cell.innerText;
      console.log(code);
      copyToClipboard(code);
      setTimeout(() => {
        console.log("Delayed for 5 second.");
        e.target.classList.remove("copied");
      }, 3000);
    });

    cell.appendChild(newDiv);
  }
});

/// ページの一番上へ行くボタン
const gotop = () => {
  console.log("go top");
  document.body.scrollIntoView({
    behavior: "smooth",
  });
};
