/*
-------------------------------------------------------------------------------
Enum
-------------------------------------------------------------------------------
Notes:
- An enum variant may either be unit-like or c-like structures.
- Each names are differents and independent.
- Type aliases is be useful when the enum name's is too long or too generic
*/

use std::time::Duration;

#[derive(Debug)]
struct PagePosition {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct PageLoadedEvent {
    content: &'static str,
    took_time_ms: Duration,
}

#[derive(Debug)]
struct PageContentCopiedEvent {
    content: &'static str,
    position: PagePosition,
}

#[derive(Debug)]
struct PageContentPastedEvent {
    content: &'static str,
    position: PagePosition,
}

#[derive(Debug)]
struct PageElementClickedEvent {
    id: &'static str,
    tag: &'static str,
    position: PagePosition,
}

#[derive(Debug)]
enum PageEventType {
    PageLoaded(PageLoadedEvent),
    PageContentCopied(PageContentCopiedEvent),
    PageContentPasted(PageContentPastedEvent),
    PageElementClicked(PageElementClickedEvent),
}

// Enum: Type Aliases
#[derive(Debug)]
enum AritmethicOperations {
    Add,
    Subtract,
    Division,
    Multiplication,
}

type MathOperationsAllowed = AritmethicOperations;

fn inspect(event: PageEventType) {
    match event {
        PageEventType::PageLoaded(payload) => println!(
            "[event.page.load] content={:?} took_time_ms={:?}",
            payload.content,
            payload.took_time_ms.as_secs_f64()
        ),
        PageEventType::PageContentCopied(payload) => println!(
            "[event.page.content.copied] content={:?} position.x={:?} position.y={:?}",
            payload.content, payload.position.x, payload.position.y
        ),
        PageEventType::PageContentPasted(payload) => println!(
            "[event.page.content.pasted] content={:?} position.x={:?} position.y={:?}",
            payload.content, payload.position.x, payload.position.y
        ),
        PageEventType::PageElementClicked(payload) => println!(
            "[event.page.element.clicked] id={:?} tag={:?} position.x={:?} position.y= {:?}",
            payload.id, payload.tag, payload.position.x, payload.position.y
        ),
    }
}

fn main() {
    let page_load_event: PageEventType = PageEventType::PageLoaded(PageLoadedEvent {
        content: "<!DOCTYPE HTML><html><body><h1>Enums - Rust By Example<h1/></body></html>",
        took_time_ms: Duration::from_millis(2_569),
    });

    let page_content_copied_event: PageEventType = PageEventType::PageContentCopied(PageContentCopiedEvent {
        content: "The enum keyword allows the creation of a type which may be one of a few different variants.",
        position: PagePosition { x: 10.0, y: 17.5 },
    });

    let page_content_pasted: PageEventType = PageEventType::PageContentPasted(PageContentPastedEvent {
        content: "<script>alert('Template injection on!')</script>",
        position: PagePosition { x: 20.45, y: 65.78 },
    });

    let page_element_clicked: PageEventType = PageEventType::PageElementClicked(PageElementClickedEvent {
        id: "#signin-button",
        tag: "button",
        position: PagePosition { x: 120.32, y: 156.65 },
    });

    inspect(page_load_event);
    inspect(page_content_copied_event);
    inspect(page_content_pasted);
    inspect(page_element_clicked);

    // Enum: Type Aliases
    let addition: MathOperationsAllowed = MathOperationsAllowed::Add;
    println!("[enum.type.aliases] addition={:?}", addition);

    let subtraction: MathOperationsAllowed = MathOperationsAllowed::Subtract;
    println!("[enum.type.aliases] subtraction={:?}", subtraction);

    let division: MathOperationsAllowed = MathOperationsAllowed::Division;
    println!("[enum.type.aliases] division={:?}", division);

    let multiplication: MathOperationsAllowed = MathOperationsAllowed::Multiplication;
    println!("[enum.type.aliases] multiplication={:?}", multiplication);
}
