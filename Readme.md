# Bnuuy_Scratch

This is a simple project for my portfolio to demonstrate my ability to deploy a React/NodeJs frontend implementing a simple Rust->WASM lexer.

## Lessons Learned for future users

1. When building from webpack build with the --target web and use the init() method to start the WASM VM.
2. When trying to render text and a cursor in a browser using a framework, it's probably easier and nicer to draw graphics using a canvas instead of trying to fiddle with drawing a cursor with css/html components and animating them with javascript.
3. Canvases suck because they use css and their own measurements, and they interact. I have 'resolved' this by making it set itself to the width of the parent div each render. It would have been faster to draw the image in WASM from the start by passing in conditionals to some lower level graphic library, since I likely lose the time I make up by using WASM.
4. Future me -> When rendering lines of text, consider them in units of line instead of foreach token.


## React Front end

| Field         | Data          |
| --            | --            |
| Font          |  [Fira Code](https://fonts.google.com/specimen/Fira+Code?query=fira) |




## To do 
- [x] Write Lexer
- [x] Bridge to WASM
- [ ] Host simple interface
- [ ] Build React FrontEnd
- [ ] Allow JS interpretation of Token stream

## Live example
- Currently undeployed
