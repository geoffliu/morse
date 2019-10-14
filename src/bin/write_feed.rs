use std::fs::File;
use xml::writer::{EmitterConfig, Result, XmlEvent};

fn main() -> Result<()> {
    let mut feed = File::create("publish/feed.rss")?;
    let mut writer = EmitterConfig::new().perform_indent(true).create_writer(&mut feed);

    writer.write(XmlEvent::start_element("rss"))?;
    writer.write(XmlEvent::start_element("channel"))?;

    writer.write(XmlEvent::start_element("title"))?;
    writer.write(XmlEvent::characters("Morse Code Bootcamp"))?;
    writer.write(XmlEvent::end_element())?;

    let episodes = morse::plan::get_lesson_plan();
    for (i, lesson) in episodes.iter().enumerate() {
        writer.write(XmlEvent::start_element("item"))?;

        writer.write(XmlEvent::start_element("title"))?;
        writer.write(XmlEvent::characters(lesson.title.as_str()))?;
        writer.write(XmlEvent::end_element())?;

        let filename = format!("https://geoffliu.me/morse/lesson{:03}.mp3", i + 1);
        writer.write(XmlEvent::start_element("enclosure").attr("url", &filename))?;
        writer.write(XmlEvent::end_element())?;

        writer.write(XmlEvent::end_element())?;
    }

    writer.write(XmlEvent::end_element())?;
    writer.write(XmlEvent::end_element())?;
    Ok(())
}
