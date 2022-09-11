# Absolute typesetting

Absolute typesetting in contrast to relative typesetting is trying to layout relatively typeset blocks onto a list of pages.
If a block doesn't fit on the current page another page is introduced.
For a trvial document that only contains simple elements a single iteration is enough.
When having forward references we need mutliple iterations.
A forward reference is a reference to a part of the document we have not yet laid out and thus do not know its location or context.
For example we might want to reference a page number that an element is on and that is yet to be laid out.
We need to first know on which page the element is before we can print the number of the page.
Unfortunately we only really know when the absolute typesetting is done.
Then we go ahead and replace all forward references to the page with the actual page number.
Now that in turn may change the line breaking of our text paragraph where the page number is referenced, which may lead to the paragraph not fitting on the same page.
Thus we need to iterate again over the whole absolute typesetting process to ensure that everything fits on its page.
That whole process may of course lead to the page number of an element to change.
Thus we need to do the whole process as long as something changes.
