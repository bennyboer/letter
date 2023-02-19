# Layout module

The `layout` module is responsible for actually laying out the elements of a document structure to a list of pages.
It is **not** responsible for rendering the pages - only for calculating the absolute positions of each possible element
in a document.
A possible element may be an individual character, an image, lines, ...

## Process

Laying out a document is a complex task.
To reduce its complexity, each element or node in a document structure will have its own set of rules for laying out on
a page, which is called a `LayoutRule`.
A `LayoutRule` is essentially a function that takes a `LayoutContext` and a node of the `Document` and returns
a `LayoutResult`.
The `LayoutContext` contains all the information and constraints needed to lay out the node, such as the current page,
the current position, the available space, etc.
Each `LayoutRule` will mutate that `LayoutContext` to reflect the changes made to the page.
For example a text paragraph requires a certain amount of space to be laid out which will be subtracted from the
available space on the current page so that the next `LayoutRule` will know that there is less space available.
Of course a text paragraph can also span multiple pages, so the `LayoutContext` will also be updated to reflect that.

In the future it may be possible for users to define their own `LayoutRule`s using a plugin or add-on system, making it
possible to easily extend the layout capabilities of the letter document layout engine.

### The problem of forward references

The biggest problem with this layout approach is that some nodes in a document structure may not know their exact size
at layout time.
For example a text paragraph that contains a forward reference - say a page number of a node that is not yet laid out.
In this case the result of the documents layout will be flagged as `unstable` since it may change when the forward
reference is resolved.
When a document layout is flagged as `unstable` it will need to be entirely re-laid out in a second pass.
In the second layout pass we already know the page number to render instead of the forward reference, so we can now
calculate the exact size of the text paragraph and lay it out correctly.
That page number has not been known beforehand and thus the elements that habe been laid out afterwards may also need to
be re-laid out since there may be less space available on the current page.
That in turn may cause forward references to be invalidated again, which will cause another re-layout pass.
Imagine that a page number of an element is now shifted from page 5 to page 6 due to the change in size of the one text
paragraph that contains the forward reference.
When laying out the document again, we may see that the forward reference is still on page 5 which means that our layout
does not need to be laid out again - since there is no change in size.
That is the moment when the document layout is considered `stable` and we can stop the layout process.

### The problem of "not enough space" in the current constraints

When laying out a node of the document, say a paragraph, we may encounter a situation where there is not enough space
available on the current page (or more specifically the layout constraints) to lay out the paragraph.
In this case we need to split the paragraph into two parts, one that fits on the current layout constraints and one that
does not.
This will work for a paragraph. There may be other nodes like images that cannot be split.
These are simply put on the next page (layout constraints).

But how are the next layout constraints determined?
When having a simple single-column page layout the next layout constraints are mostly the next page.
When having a multi-column layout, the next layout constraints are the next column on the current page.
Thus the ´LayoutContext´ needs some kind of rule to apply when encountering the "not enough space" situation.
We call this a `LayoutConstraintRule`.
This rule can be set via the Letter stylesheets (`*.lst`) `layout` property.

```
section.multi-column {
    layout: column;
    column-count: 2;
}
```

When encountering the "not enough space" situation in a `<section>` with class `multi-column` the available space will
be divided into two and the according `LayoutConstraintRule` set to the `LayoutContext`.
