use leptos::{*, html::Input};
use calamine::{open_workbook, Reader, Xlsx, DataType};

#[component]
pub fn TypeArea(cx: Scope, send: Action<String, Result<String, ServerFnError>>) -> impl IntoView {
    let input_ref = create_node_ref::<Input>(cx);
    let file_ref = create_node_ref::<Input>(cx);

    view!{ cx,
        <div class="h-24 w-full fixed bottom-0 flex justify-center items-center p-5 bg-white border-t border-gray-300">
           <form class="w-full flex justify-center items-center gap-4" on:submit=move |ev| {
                ev.prevent_default();
                let input = input_ref.get().expect("input to exist");
                let file = file_ref.get().expect("file to exist");
                let mut input_text = input.value();

                if file.value() != "" {
                    let mut workbook: Xlsx<_> = open_workbook(file.value()).expect("open workbook");
                    if let Some(Ok(range)) = workbook.worksheet_range("Sheet1") {
                        for row in range.rows() {
                            for cell in row.iter() {
                                if let DataType::String(s) = cell {
                                    input_text.push_str(s);
                                    input_text.push_str("\n");
                                }
                            }
                        }
                    }
                }

                send.dispatch(input_text);
                input.set_value("");
           }
           >
                <input type="file" accept="excel/*" node_ref=file_ref />
                <input class="w-2/3 p-4 border border-gray-300 rounded-full" type="text" placeholder="Enter your prompt" node_ref=input_ref/>
                <input class="h-full p-4 bg-blue-500 hover:bg-blue-700 text-white rounded-full cursor-pointer" type="submit"/>
           </form>
        </div>
    }
}