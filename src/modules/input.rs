// 副作用のある関数はinput.rsとoutput.rs以下に置く。

pub mod fetch_google_forms;

pub async fn main(args: Args) {
    dbg!("input.rs");
    fetch_google_forms::main(args.client_id, args.client_secret, args.form_id);
}
