# Letter script format

The `script` subproject of the Letter document layout engine contains the definition and parser for the Letter script markup language.
It is used to define the contents of a Letter document and inherited mostly from XML and Markdown.
The idea is to be able to use something like Markdown to write your document contents while still being able to define details as extensively as with XML.

A short example:

```
# My document title

This is a normal paragraph. You can also make parts of the text **bold** or *cursive*.
More is possible by using letter script extensions like [link target="https://google.com"] this link to Google [/link] or the markdown shorthand [this link to Google](https://google.com).
In fact all markdown shorthands like the above heading using the # character are replacable by their actual letter script extensions.
For headings you could also use [code][heading level=1] My document title [/heading][/code].

Styling individual parts of a paragraphs text could be done using a custom letter script extension like [code][my-extension] Some text [/my-extension][/code].
If there is a letter style format assigned to the document like the following, the text would appear in red and bold.

[code format="letter-style"]
my-extension {
  text {
    color: red
    weight: bold
  }
}
[/code]
```
