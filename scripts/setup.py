def question(question, default=None):
    """
    yes/no の質問を行う
    """
    while True:
        default_prompt = (
            ""
            if default is None
            else "(default: {default}) ".format(default="yes" if default else "no")
        )
        prompt = "{question}? (yes or no) {default_prompt}".format(
            question=question, default_prompt=default_prompt
        )
        answer = input(prompt)
        if any(answer.lower() == f for f in ["yes", "y", "1", "ye"]):
            return True
        elif any(answer.lower() == f for f in ["no", "n", "0"]):
            return False
        elif default is not None:
            return default
        else:
            print("Please enter yes or no")


def exit_on_fail():
    print("Aborting ...")
    quit()


enable_ssl = question("Do you want to enable SSL/TLS")
print("Enable SSL/TLS" if enable_ssl else "Disable SSL/TSL")

if enable_ssl:
    print("enable SSL/TLS")

    prompt = """
    TLS/SSL の証明書を発行し，cert.pem, key.pem という名前にして，
    プロジェクトのルートディレクトリに置く．
    TLS/SSL を有効にしないなら不要．
    Done already"""

    # print(prompt)
    if not question(prompt):
        exit_on_fail()

    # サンプルの db が docs 以下にあるので，それを持ってくる．

    prompt = """
    初期ユーザを追加する．
    ユーザ名が foo，パスワードが bar のユーザを追加したい場合は，
    db/users directory に ファイル名が foo で，
    以下のような JSON が書かれたファイルを配置する．
    Done already"""

    # print(prompt)
    if not question(prompt):
        exit_on_fail()

    prompt = """
    .env.template を参考に，.env を生成する
    Done already"""

    # print(prompt)
    if not question(prompt):
        exit_on_fail()
    prompt = """
    cargo で backend を実行する
    Done already"""

    # print(prompt)
    if not question(prompt):
        exit_on_fail()
    prompt = """
    access https://127.0.0.1:8443/ on your browser.
    Done already"""

    # print(prompt)
    if not question(prompt):
        exit_on_fail()

else:
    print("enable SSL/TLS" if enable_ssl else "disable SSL/TSL")
