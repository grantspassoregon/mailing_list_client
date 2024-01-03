#![allow(non_snake_case)]
use crate::prelude::*;
use dioxus::prelude::*;
use spreadsheet::prelude::*;

pub fn Home(cx: Scope) -> Element {
    let theme = use_shared_state::<Theme>(cx);
    let class = Theme::get(&theme, "background");
    let input = Theme::get(&theme, "input");
    let button = Theme::get(&theme, "button");
    let files = use_ref(cx, Files::new);
    let target_files = use_ref(cx, Files::new);
    cx.render(rsx!(HomeContent {
        class: class,
        input: input,
        button: button,
        files: files.clone(),
        target_files: target_files.clone(),
    }))
}

#[derive(Props, PartialEq)]
struct HomeContentProps {
    #[props(into)]
    class: String,
    #[props(into)]
    input: String,
    #[props(into)]
    button: String,
    #[props(into)]
    files: UseRef<Files>,
    #[props(into)]
    target_files: UseRef<Files>,
}

fn HomeContent(cx: Scope<HomeContentProps>) -> Element {
    tracing::info!("Home page drawing.");
    let source_file = use_state(cx, || String::new());
    {
        let source_file = source_file.clone();
        use_effect(cx, &cx.props.files, |files| async move {
            source_file.set(files.read().selected());
        })
    }
    let see_source_picker = use_state(cx, || true);

    let output_file = use_state(cx, || String::new());
    {
        let output_file = output_file.clone();
        use_effect(cx, &cx.props.target_files, |files| async move {
            output_file.set(files.read().selected());
        })
    }
    let see_target_picker = use_state(cx, || true);
    let message = use_state(cx, || String::new());

    cx.render(rsx!(
        div  {
            class: cx.props.class.as_str(),
            Menu {}
            Picker { 
                input: cx.props.input.clone(), 
                button: cx.props.button.clone(), 
                label: "Source file:".to_string(), 
                files: cx.props.files.clone(), 
                selected: source_file.clone(), 
                see_picker: see_source_picker.clone() 
            }
            Picker { 
                input: cx.props.input.clone(), 
                button: cx.props.button.clone(), 
                label: "Output file:".to_string(), 
                files: cx.props.target_files.clone(), 
                selected: output_file.clone(), 
                see_picker: see_target_picker.clone() 
            }

            div {
                class: "flex flex-row justify-center",
                button {
                    class: cx.props.button.as_str(),
                    onclick: move |_| {
                        mailing_list(source_file.clone(), output_file.clone(), message.clone()).unwrap();
                    },
                    "Run"
                }
            }
            message.get().as_str()
        }
    ))
}

fn mailing_list(
    source_file: UseState<String>,
    output_file: UseState<String>,
    message: UseState<String>,
) -> ClientResult<()> {
    tracing::info!("Checking target path.");
    let mut target = output_file.get().clone();
    match target.as_str() {
        "" => {
            message.set("Output path cannot be empty.".to_string());
        }
        _ => {
            if !target.contains(".csv") {
                tracing::info!("Adding .csv extension.");
                target = format!("{}.csv", target);
            }
            let mut target_path = std::path::Path::new(&target);
            let source = source_file.get();
            let path = std::path::Path::new(source);
            match path.try_exists() {
                Ok(true) => {
                    let records = CountyTaxlots::from_csv(source_file.get())?;
                    let mail = MailingList::try_from(&records)?;
                    let mut mail = MailingListExport::from(&mail);
                    mail.sort_by_key("properties");
                    let mail: Vec<MailingListExportItem> =
                        mail.records_ref().iter().rev().cloned().collect();
                    let mut mail = MailingListExport::new(mail);
                    if !target_path.has_root() {
                        let cwd = match std::env::current_dir() {
                            Ok(path) => path,
                            Err(_) => "./".into(),
                        };
                        target = format!("{}\\{}", cwd.display(), target);
                        target_path = std::path::Path::new(&target);
                    }
                    match mail.to_csv(target_path) {
                        Ok(_) => message.set(format!(
                            "Mailing list output to {}",
                            target_path.display()
                        )),
                        Err(e) => message.set(format!(
                            "Failure writing to output file location: {}",
                            e.to_string()
                        )),
                    };
                }
                Ok(false) => {
                    message.set("Invalid source file selected.".to_string());
                }
                Err(e) => {
                    message.set(format!("Error reading source file: {}", e.to_string()));
                }
            }
        }
    }
    Ok(())
}
