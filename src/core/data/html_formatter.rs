use std::fmt::Write;

use comrak::{create_formatter, nodes::NodeValue};

create_formatter!(HtmlFormatter, {
    NodeValue::Link(ref node) => |context, entering| {
        let node_url = node.url.clone();
        let is_external = node_url.starts_with("http");

        let external_icon = "<span class=\"tw-link-icon\"><svg width=\"100%\" height=\"100%\" viewBox=\"0 0 60 60\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\"><path d=\"M22.5006 10.5H17.1006C13.7403 10.5 12.0589 10.5 10.7754 11.154C9.64641 11.7292 8.72919 12.6464 8.15397 13.7754C7.5 15.0589 7.5 16.7403 7.5 20.1006V42.9006C7.5 46.2609 7.5 47.9403 8.15397 49.2237C8.72919 50.3526 9.64641 51.2715 10.7754 51.8466C12.0576 52.5 13.737 52.5 17.0907 52.5H39.9093C43.263 52.5 44.94 52.5 46.2222 51.8466C47.3511 51.2715 48.2715 50.3517 48.8466 49.2228C49.5 47.9406 49.5 46.263 49.5 42.9093V37.5M52.5 22.5V7.5M52.5 7.5H37.5M52.5 7.5L31.5 28.5\" stroke=\"currentColor\" stroke-width=\"1.5\" stroke-linecap=\"round\" stroke-linejoin=\"round\"></path></svg></span>";

        if entering && !is_external {
            context.write_fmt(format_args!("<a href=\"{node_url}\"><span class=\"tw-secondary-link tw-link-size-md\"><span class=\"tw-link-text\">"))?;
        } else if entering && is_external {
            context.write_fmt(format_args!("<a href=\"{node_url}\" target=\"_blank\"><span class=\"tw-secondary-link tw-link-size-md\"><span class=\"tw-link-text\">"))?;
        } else if is_external {
            context.write_fmt(format_args!("</span>{external_icon}</span></a>"))?;
        } else {
            context.write_str("</span></span></a>")?;
        }
    },
});
