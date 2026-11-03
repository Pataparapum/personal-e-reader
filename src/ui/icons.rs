use iced::widget::svg;
use iced::Element;

use crate::status::Wifi;

const STROKE: &str = r#"fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round""#;

fn render<'a, M: 'a>(markup: String, size: f32) -> Element<'a, M> {
    svg(svg::Handle::from_memory(markup.into_bytes()))
    .width(size)
    .height(size)
    .into()
}

pub fn battery<'a, M: 'a>(percent: u8, charging: bool) -> Element<'a, M> {
    let fill = 15.0 * f32::from(percent.min(100)) / 100.0;
    let bolt = if charging {
        r#"<polygon points="13,4 7,12.5 11,12.5 10,20 17,11 13,11" fill="white" stroke="black" stroke-width="1"/>"#
    } else {
        ""
    };

    render(
        format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
<rect x="1" y="5" width="19" height="14" rx="2" {STROKE}/>
<rect x="21" y="9" width="2" height="6" fill="black"/>
<rect x="3" y="7" width="{fill:.1}" height="10" fill="black"/>
{bolt}
</svg>"#
        ),
        26.0,
    )
}

pub fn wifi<'a, M: 'a>(wifi: Wifi) -> Element<'a, M> {
    let (level, off) = match wifi {
        Wifi::Off => (3, true),
        Wifi::On(l) => (l.min(3) as usize, false),
    };

    let arcs = [
        "M8.46 15.46 A5 5 0 0 1 15.54 15.46",
        "M5.64 12.64 A9 9 0 0 1 18.36 12.64",
        "M2.81 9.81 A13 13 0 0 1 21.19 9.81",
    ];

    let mut body = String::new();
    for path in arcs.iter().take(level) {
        body.push_str(&format!(r#"<path d="{path}" {STROKE}/>"#));
    }
    if off {
        body.push_str(&format!(r#"<path d="M3 3 L21 21" {STROKE}/>"#));
    }

    render(
        format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
<circle cx="12" cy="19" r="2" fill="black"/>
{body}
</svg>"#
        ),
        22.0,
    )
}

pub fn usb<'a, M: 'a>() -> Element<'a, M> {
    render(
        format!(
             r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
<path d="M12 5 V20 M12 14 L6 10 V8 M12 11 L18 8 V7" {STROKE}/>
<circle cx="12" cy="20" r="2" fill="black"/>
<circle cx="6" cy="6.5" r="1.8" fill="black"/>
<rect x="16" y="4" width="4" height="3" fill="black"/>
<polygon points="12,1 9.5,5 14.5,5" fill="black"/>
</svg>"#
        ),
        22.0,
    )
}

/// Placeholder: rayos radiales, Para usar el log real, ver la nota abajo
pub fn logo<'a, M:'a>(size: f32) -> Element<'a, M> {
    let mut rays = String::new();
    for i in 0..12 {
        let a = (i as f32) * 30.0_f32.to_radians();
        let outer = if i % 2 == 0 { 46.0 } else { 36.0 };
        let (x1, y1) = (50.0 + 12.0 * a.sin(), 50.0 - 12.0 * a.cos());
        let (x2, y2) = (50.0 + outer * a.sin(), 50.0 - outer * a.cos());
        rays.push_str(&format!(
            r#"<line x1="{x1:.1}" y1="{y1:.1}" x2="{x2:.1}" y2="{y2:.1}"/>"#
        ));
    }

    render(
        format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
<g stroke="black" stroke-width="8" stroke-linecap="round">{rays}</g>
</svg>"#
        ),
        size,
    )    
}

// render_bytes(&include_bytes!("../../assets/logo.svg")[..], size)
// añadiendo esta función auxiliar junto a render:
// fn render_bytes<'a, M: 'a>(bytes: &'static [u8], size: f32) -> Element<'a, M> {
//    svg(svg::Handle::from_memory(bytes)).width(size).height(size).into()
// }