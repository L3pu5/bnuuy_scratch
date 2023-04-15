import React, { useState, useRef, useEffect } from 'react';
import PsudoDir from '../psudoDir/PsudoDir';
import './Scratchpad.css';
import TokenRenderer from './TokenRenderer/TokenRenderer';
import init, {build_tokens} from "lexer";


const welcome_text = "Welcome to \"bunny_scratch\" by L3Pu5 Hare!\n| Everything here has been written from scratch using React/JS and 🦀Rust/Webpack🕸️.\n|  Tokenisation is done with rust, and this simple text editor is written in React/JS.\n\n<-- FileSystem| (Editor) |Settings -->\n\nFileSytem\n| This is your temporary in-browser (filesystem).\n| Do not save anything important in (/temp), it will be purged when the browser tab dies. VERY temporary.\nEditor\n| This is the centre panel. Excepting this splash screen, you can edit files in here!.\nSettings\n| This area lets you tweak the syntax highlighting and settings such as the tabwidth, etc.\n| You can also use the export feature to save to your local disk.";

export default function Scratchpad() {
  const [text, setText] = useState(welcome_text);
  const [tokens, setTokens] = useState([]);
  const reset = useRef(false);
  
  const focus_element = useRef(0); //Editor = 0, Nav = 1
  const setFocus = (i) => {focus_element.current = i};

  const resetText = (newText) => {
    setText(newText)
    reset.current = true;
  } 

  useEffect( () => {
  }
  , [text])

  async function createTokens (string_input) {
     (init().then( (_exports) => {
      var newTokens = JSON.parse(build_tokens(string_input));
      if( tokens.length === newTokens.length){
        if(reset.current === true){
          setTokens(newTokens);
          reset.current = false;  
        }
      } else{
        setTokens(newTokens);
      }
    }))
  }

  createTokens(text);

  const handleFileClick = (text) =>  {
    setText(text);
  }

  return (
    <>
    <div className="Editor"> 
      <PsudoDir handleClick={handleFileClick}></PsudoDir>
      <div className='mainScratch'>
        <TokenRenderer tokens={tokens} setText={resetText} rawText={text} setFocus={setFocus}> </TokenRenderer>
      </div>
    </div>
    </>
  )
}
