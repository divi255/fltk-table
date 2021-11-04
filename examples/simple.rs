use fltk::{
    app, enums,
    prelude::{GroupExt, WidgetExt},
    window,
};
use fltk_table::{SmartTable, TableOpts};

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = window::Window::default().with_size(800, 600);

    let mut table = SmartTable::default(TableOpts {
        rows: 30,
        cols: 15,
        ..Default::default()
    })
    .with_size(790, 590)
    .center_of_parent();
    
    // the default is false
    table.editable(true);

    wind.end();
    wind.show();

    // set the value at the row,column 4,5 to "another"
    table.set_cell_value(3, 4, "another");

    assert_eq!(table.cell_value(3, 4), "another");

    // To avoid closing the window on hitting the escape key
    wind.set_callback(move |_| {
        if app::event() == enums::Event::Close {
            app.quit();
        }
    });

    app.run().unwrap();
}
