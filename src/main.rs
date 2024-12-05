use std::io::BufWriter;

//use html5ever::driver::ParseOpts;
//use html5ever::serialize::{serialize, HtmlSerializer};
//use html5ever::{local_name, parse_fragment, QualName};
//use html5ever::{namespace_url, ns, parse_document};
//use html5ever::tendril::StrTendril;
//use html5ever::tendril::TendrilSink;
use markup5ever_rcdom::{Handle, NodeData, RcDom, SerializableHandle};
use xml5ever::driver::parse_document;
use xml5ever::serialize::serialize;
use xml5ever::tendril::TendrilSink;
use xml5ever::{namespace_url, ns};

fn walk(indent: usize, handle: &mut Handle) {
    let node = handle;
    for _ in 0..indent {
        print!(" ");
    }
    match node.data {
        NodeData::Document => println!("#Document"),

        NodeData::Doctype {
            ref name,
            ref public_id,
            ref system_id,
        } => println!("<!DOCTYPE {name} \"{public_id}\" \"{system_id}\">"),

        NodeData::Text { ref contents } => {
            println!("#text: {}", contents.borrow().escape_default())
        }

        NodeData::Comment { ref contents } => println!("<!-- {} -->", contents.escape_default()),

        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            print!("<{}", name.local);
            for attr in attrs.borrow().iter() {
                assert!(attr.name.ns == ns!());
                print!(" {}=\"{}\"", attr.name.local, attr.value);
            }
            println!(">");
        }

        NodeData::ProcessingInstruction { .. } => unreachable!(),
    }

    for child in node.children.borrow_mut().iter_mut() {
        walk(indent + 4, child);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let sink = todo!();
    //let ops = todo!();
    //let ctx_name = todo!();
    //let ctx_attr = todo!();
    //parse_fragment(sink, opts, ctx_name, ctx_attr);

    let text = r#"<div className="foo"></div>"#.to_owned();

    //let dom = parse_fragment(
    //    RcDom::default(),
    //    Default::default(),
    //    QualName::new(None, ns!(html), local_name!("div")),
    //    Default::default(),
    //)
    //.from_utf8()
    //.read_from(&mut text.as_bytes())
    //.unwrap();

    let mut dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut text.as_bytes())?;

    walk(0, &mut dom.document);
    let mut s = BufWriter::new(Vec::new());
    serialize(
        &mut s,
        //&SerializableHandle::from(dom.document.children.borrow()[0].clone()),
        &SerializableHandle::from(dom.document),
        Default::default(),
    )?;
    let s = String::from_utf8(s.into_inner()?)?;
    println!("{}", s);

    Ok(())
}
