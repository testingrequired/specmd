use markdown::{Constructs, mdast::Node, to_mdast};

use crate::span::{NO_SPAN, Spanned};

pub type SpannedContent = Spanned<String>;
pub type SpannedCodeBlock = Spanned<SpannedContent>;

pub fn extract_frontmatter(input: impl AsRef<str>) -> Option<SpannedContent> {
    let options = &mut markdown::ParseOptions::default();

    options.constructs = Constructs {
        frontmatter: true,
        ..Default::default()
    };

    let frontmatter = to_mdast(input.as_ref(), options)
        .unwrap()
        .children()
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .find(|node| matches!(node, Node::Yaml(_)))
        .and_then(|node| match node {
            Node::Yaml(yaml) => {
                let span = yaml
                    .position
                    .as_ref()
                    .map(|pos| {
                        let start = pos.start.offset + 4;
                        let end = pos.end.offset - 4;

                        start..end
                    })
                    .unwrap_or(NO_SPAN);

                let text = &yaml.value;

                return Some((text.clone(), span));
            }
            _ => None,
        });

    frontmatter
}

/// Extract matching lang code blocks from a markdown string.
pub fn extract_codeblocks(
    input: impl AsRef<str>,
    target_lang: impl AsRef<str>,
) -> Vec<SpannedCodeBlock> {
    let mut results = vec![];

    let md_nodes: Vec<Node> = to_mdast(input.as_ref(), &markdown::ParseOptions::default())
        .unwrap()
        .children()
        .cloned()
        .unwrap_or_default();

    for md_node in md_nodes {
        if let Node::Code(codeblock) = &md_node {
            let position = codeblock.position.as_ref().unwrap();
            let start = position.start.offset;
            let end = position.end.offset;

            let text = codeblock.value.clone();
            let text_start = (3 + 1) + target_lang.as_ref().len() + start;
            let text_end = text_start + text.len();

            if let Some(lang) = &codeblock.lang
                && lang == target_lang.as_ref()
            {
                results.push(((text, text_start..text_end), start..end));
            }
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn extract_codeblock_matching_lang() {
        let input = textwrap::dedent(
            "
```javascript
const foo = 123;
```

```test_lang
TEST
```
            ",
        );

        assert_eq!(
            vec![((String::from("TEST"), 50..54), 37..58)],
            extract_codeblocks(input, "test_lang")
        );
    }

    #[test]
    fn extract_multiple_codeblocks_matching_lang() {
        let input = textwrap::dedent(
            "
```javascript
const foo = 123;
```

```test_lang
TEST
```

```
TEST
```

```test_lang
TEST TEST
```
            ",
        );

        assert_eq!(
            vec![
                ((String::from("TEST"), 50..54), 37..58),
                ((String::from("TEST TEST"), 87..96), 74..100)
            ],
            extract_codeblocks(input, "test_lang")
        );
    }

    #[test]
    fn doesnt_exact_nonmatching_lang() {
        let input = textwrap::dedent(
            "
            ```javascript
            const foo = 123;
            ```

            ```test_lang
            TEST
            ```
            
            ```
            TEST
            ```

            ```test_lang
            TEST TEST
            ```
            ",
        );

        let expected: Vec<SpannedCodeBlock> = vec![];

        assert_eq!(expected, extract_codeblocks(input, "nonmatching_lang"));
    }
}
