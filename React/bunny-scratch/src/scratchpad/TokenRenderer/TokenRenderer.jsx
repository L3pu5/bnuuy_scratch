import React, { useState, useRef } from 'react'
import BottomBar from './BottomBar';
import Cursor from './Cursor';
import './TokenRenderer.css';
import RenderCanvas from './RenderCanvas';

export default function TokenRenderer(props) {
    const [index, setIndex] = useState(0);

    var lines = props.rawText.split('\n');
    var lineWidths = lines.map( (l) => l.length);

    if(index >= props.rawText.length){
      setIndex(0);
    }

    const seekUpLine = () => {
      var idx = 0;
      for (let i = 0; i < lineWidths.length; i++) {
        var lineW = lineWidths[i];
        idx += lineW;
        if(idx >= index){
          //We need the line before
          if(i > 0){
            //Previous line is 
            var prev = lineWidths[i-1];

            //Start of this line in index is
            var lineStartIndex = idx - lineW;
            if(prev === 0){
              setIndex(lineStartIndex+1);
              return;
            }

            var chars_into_line = index - lineStartIndex;

            if(prev >= (index - lineStartIndex)){
              //IDX - Index = amount missing
              // LineW - (IDx - Index) = progrees
              setIndex(lineStartIndex-prev+chars_into_line-1);
              return;
            }
            else{
              setIndex(lineStartIndex-1);
            }
          }
          return;
        }
      }
    }

    const seekDownLine = () => {
      var idx = 0;
      var next = false;
      for (let i = 0; i < lineWidths.length; i++) {
        var lineW = lineWidths[i];
        idx += lineW;
        if(next == true){
          //Are we only a link?
          if(lineW == 0){
            setIndex(idx+1);
            return;
          }

          //We need this line 
            var next = lineW;
            //Start of this line in index is
            var lineStartIndex = idx - lineW;
            var prevStart = lineStartIndex - lineWidths[i-1];
            var chars_into_line = index - prevStart;
            console.log(chars_into_line);
            if (next >= (chars_into_line)) {
              //IDX - Index = amount missing
              // LineW - (IDx - Index) = progrees
              console.log("Setting to: ",lineStartIndex + chars_into_line +1)//Characters missing )
              setIndex(lineStartIndex + chars_into_line+1);
              return;
            }
            else {
              setIndex(lineStartIndex + lineW +1);
              console.log("ILLEGAL");
              return;
            }
          }

        

        if(idx >= index){
          next = true;
          }
        }
      }
    

   
    //We should handle index, then infer line/col of cursor
    const incrementIndex = () => {
      var skipWhite = 1;
      var c = props.rawText.charAt(index+skipWhite);
      if( c=== '\n')
      {
        skipWhite += 1;
      }
      setIndex(index + skipWhite);
    }

        //We should handle index, then infer line/col of cursor
    const decrementIndex = () => {
          var skipWhite = -1;
          var c = props.rawText.charAt(index-1);
          var nl = 0;
          if( c=== '\n')
          {
            skipWhite -= 1;
          }
          setIndex(index + skipWhite);
          }

  var isAlpha = function (ch) {
    return /^[A-Za-z]{1,1}$/.test(ch)
  }

    const handleKeyDown = (down) => {
      down.stopPropagation();
      down.preventDefault ? down.preventDefault() : down.returnValue = false;
      //Control keys
      if (down.ctrlKey) {
        if (down.key === 'b') {
          setIndex(0);
          return;
        }

        if (down.key === "ArrowRight") {
          //Seek next word end
          var idx = 1;
          var c = props.rawText.charAt(index + idx);
          while( isAlpha(c)){
            if(idx + 1 >= props.rawText.length){
              setIndex(props.rawText.length)
              return;
            }
            idx += 1;
            c = props.rawText.charAt(index + idx);
          }
          setIndex(index + idx);
          return;
        }

        if (down.key === "ArrowLeft") {
          //Seek next word end
          var idx = 1;
          var c = props.rawText.charAt(index - idx);
          while( isAlpha(c)){
            if(idx - 1 <= 0){
              setIndex(0);
              return;
            }
            idx += 1;
            c = props.rawText.charAt(index - idx);
          }
          setIndex(index - idx);
          return;
        }
      }
      
      switch (down.key) {
        case "ArrowLeft":
          decrementIndex()
          return;
          break;
        case "ArrowRight":
          incrementIndex();
          return;
        case "ArrowUp":
          seekUpLine();
          return;
        case "ArrowDown":
          seekDownLine();
          return;
        default:
          break;
      }

      if(down.keyCode === 9){
        var newText = props.rawText.slice(0, index) + "   " + props.rawText.slice(index);
        setIndex(index + 3);
        props.setText(newText);
        return;  
      }

      if( down.keyCode >= 32 && down.keyCode <= 230){     
        if(down.key === "Delete"){
          var newText = props.rawText.slice(0, index) + props.rawText.slice(index+1);
          props.setText(newText);
          return;  
        }

        
        if(down.key === "("){
          var newText = props.rawText.slice(0, index) + "()" + props.rawText.slice(index);
          incrementIndex();
          props.setText(newText);
          return;
        }

        if(down.key === "{"){
          var newText = props.rawText.slice(0, index) + "{}" + props.rawText.slice(index);
          incrementIndex();
          props.setText(newText);
          return;
        }

        if(down.key === "["){
          var newText = props.rawText.slice(0, index) + "[]" + props.rawText.slice(index);
          incrementIndex();
          props.setText(newText);
          return;
        }

        if(down.key === "\'"){
          var newText = props.rawText.slice(0, index) + "\'\'" + props.rawText.slice(index);
          incrementIndex();
          props.setText(newText);
          return;
        }

        if(down.key === "\""){
          var newText = props.rawText.slice(0, index) + "\"\"" + props.rawText.slice(index);
          incrementIndex();
          props.setText(newText);
          return;
        }

        var newText = props.rawText.slice(0, index) + down.key + props.rawText.slice(index);
        incrementIndex();
        props.setText(newText);
        return;
      }
      //enter
      if(down.keyCode === 13){
        var newText = props.rawText.slice(0, index) + '\n' + props.rawText.slice(index);
        incrementIndex();
        console.log(newText);
        props.setText(newText);
        return;
      }
      //backspace
      if(down.keyCode === 8){
        var newText = props.rawText.slice(0, index-1) + props.rawText.slice(index);
        decrementIndex();
        props.setText(newText);
        return;
      }
    }



    return (
      <>
        <div className="ContentHolder" tabIndex={-1} onKeyDown={handleKeyDown}>
          <RenderCanvas tokens={props.tokens} cursorIndex={index} rawText ={props.rawText}></RenderCanvas>
        </div>
      </>
  )
}
