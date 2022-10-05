# Auto-Doc
A tool for creating code documentation and manuals for software/codefiles

NOTE:
This is the first thing I've ever written in Rust, my motivation for writing it is equal parts my desire for the product
and my desire to learn through the process of building something. As such, much of the code will likely be sloppy, not 
best practice, inefficient, ugly. I am doing my best to avoid this but I have very little exposure to the language. Should
anyone review this, feel free to tell me what I've done wrong, what could be better. Cheers. 

While I have learned to code, one thing I have taken away is that when looking at other peoples code, even if it is well-commented,
it can be very difficult to understand what a given function, file, class, etc,. does in the context of the *entire* project. It was 
my intent in writing this to create a tool that would create documentation for a project that is as easily navigable as a wiki.

What this project does is trawl a directory for code files, then create a wiki style "article" for each one that contains
some automatically generated documentation where possible, as well as creates places for the user to put in comments to give
broad stroke explanations of what the code does. These articles will also link to other files whenever they appear in one another.
For example. if you import a function 'bar' from a file 'foobar.py,' 'bar' will be hyperlinked to its definition in the 'foobar' 
document. 

This program is optimized for Obsidian notes, for the best experience you should open the generated folder in Obsidian for viewing. 
However, the output is nothing but markdown files, so feel free to use your editor/viewer of choice, but note that it will likely not work as intended there. 
