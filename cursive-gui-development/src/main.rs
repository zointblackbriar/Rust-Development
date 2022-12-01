extern crate cursive; 
use cursive::Cursive;
use cursive::views::{Dialog, TextView, Checkbox, EditView};
use cursive::traits::Identifiable; // for .with_id() and .call_on_id()


struct CatSomeOption<'a> {
    message: &'a str 
    dead: bool,
}

fn input_step(sampleGUI: &mut Cursive){
    sampleGUI.add_layer(
        Dialog::new().title("Please fill out the form for the cat")
        //setting the title
        .content(ListView::new().child("Message: ", EditView::new().with_id("message")))
        .child("IsActive?", Checkbox::new().with_id("activate")),
    )
    .button("OK", |s| {
        let message = s.call_on_id("message", |t: &mut EditView| t.get_content())
    })
}

fn main() {
    let mut sampleParam = Cursive::default();

    let sampleText = "hello text for GUI application";
    sampleParam.add_layer(Dialog::around(TextView::new(sampleText)).button("OK", |s| s.quit()));
    sampleParam.run();
    // println!("Hello, world!");
}
