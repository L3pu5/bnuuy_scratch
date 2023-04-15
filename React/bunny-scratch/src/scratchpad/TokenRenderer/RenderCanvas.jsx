import React, { useEffect, useRef, useState } from 'react'
import './TokenRenderer.css'
import BottomBar from './BottomBar.jsx'

export default function RenderCanvas(props) {
    const canvas = useRef(null)
    const width = useRef(null);
    const [colours, setColours] = useState(
        {
            "NUMBER" : "#0F0",
            "STRING" : "#ffb347",
            "GRAMMAR": "#F00",
            "DEFAULT": "#CCC",
        }
    )
    
    const targetHeight = 600;
    var maxLineChars = Math.floor(width.current/8.4);

    const charWidth = 8.4;
    const charHeight = 17;

    var col = 0;
    var ln = 0;
    
    // Attach the event listener to the window object
    window.addEventListener('resize', handleResize);

    function handleResize() {
        if(canvas.current === null){
            canvas.current = (document.getElementById("RENDERTARGET"))
        }
        width.current = canvas.current.parentNode.clientWidth; 
        canvas.current.width = width.current;
        canvas.current.height = 600;
        maxLineChars = Math.floor(width.current/8.4);
        draw();
    }

    //Resize canvas
    useEffect( () => {
        if(canvas.current === null)
            return;
        var context = canvas.current.getContext("2d");
        width.current = canvas.current.parentNode.clientWidth; 
        canvas.current.width = width.current;
        canvas.current.height = 600;
        //size 14 FiraCode = 8.4px width, ~17px height
        //canvas.current.width = targetWidth
        context.font = "14px 'Fira Code'";
        context.fillStyle = "#CCC";
    }, [canvas])

    //Takes props index.    
    useEffect( () => {
        canvas.current = (document.getElementById("RENDERTARGET"))
        if(props.tokens[0] === undefined)
        return;

        draw();

    },)

    const draw = () => {
        if(canvas.current === null)
             return;
        clearCanvas();
        drawAllTokens();
        //drawCursor(props.x,props.y);
    }


    const drawAllTokens = () => {
        var chars_in_line = 0;
        var line = 0;
        var trueLine = 0;
        var idx = 0;
        var targetIdx = 0;
        if(props.cursorIndex >= props.rawText.length){
            targetIdx = 0;
        }
        else{
            targetIdx = props.cursorIndex;
        }

        var renderedCursor = false;
        console.log(props.tokens);
        for (let index = 0; index < props.tokens.length; index++) {
            var token = props.tokens[index];
            if(token.kind === "STRING"){
                if(token.manipulated === undefined){
                    token.manipulated = true;
                    //Hack the token by grabbing the guarder characters
                    token.text = props.rawText.charAt(idx) +token.text + props.rawText.charAt(idx + token.text.length + 1);
                }
            }

            if(token.kind === "NEWLINE"){
                chars_in_line = 0;
                trueLine += 1;
                line += 1;
                if((idx) === targetIdx && !renderedCursor){
                    col = chars_in_line + (targetIdx - idx);
                    ln = trueLine;
                    drawCursor((chars_in_line + (targetIdx - idx)) *8.4, (line)*17)
                    renderedCursor = true;
                }
                idx += 1;
                if((idx) === targetIdx && !renderedCursor){
                    col = chars_in_line + (targetIdx - idx);
                    ln = trueLine;
                    
                    drawCursor((chars_in_line + (targetIdx - idx)) *8.4, (line)*17)
                    renderedCursor = true;
                }
                continue;
            }
            if(chars_in_line + token.text.length > maxLineChars){
                chars_in_line = 0;
                line += 1;
            }
            //console.log("CURSORINDEX ", targetIdx, " IDX ", idx);
            if((idx + token.text.length) > targetIdx && !renderedCursor){
                col = chars_in_line + (targetIdx - idx);
                ln = trueLine;    
                drawCursor((chars_in_line + (targetIdx - idx)) *8.4, (line)*17)
                renderedCursor = true;
            }
            idx += token.text.length;
            drawToken(token, chars_in_line*8.4, (line +1)*17);
            chars_in_line += token.text.length;
        }
    }

    const drawCursor = (x, y) => {
        if(canvas.current === null){
            canvas.current = (document.getElementById("RENDERTARGET"))
        }
        var context = canvas.current.getContext("2d");
        context.fillStyle = "#F00"
        context.fillRect(x, y, charWidth, charHeight+2);
    }

    const clearCanvas = () => {
        if(canvas.current === null){
            canvas.current = (document.getElementById("RENDERTARGET"))
        }
        var context = canvas.current.getContext("2d");
        //console.log("Got context", context)
        context.fillStyle = "#1b1b1b"
        context.fillRect(0, 0, canvas.current.width, canvas.current.height);
    }
    
    const drawToken =(token, x, y) => {
        if(canvas.current === null){
            canvas.current = (document.getElementById("RENDERTARGET"))
            console.log("canvas is l", canvas.current)
        }
        var context = canvas.current.getContext("2d");
        context.font = "14px 'Fira Code'";
        if(colours[token.kind] == null){
            context.fillStyle = "#CCC";
        }else{
            context.fillStyle = colours[token.kind];
        }
        context.fillText(token?.text, x ,y);
    } 

    const bottomBar = () => {
        return <BottomBar idx={props.cursorIndex} col={col} line={ln}></BottomBar> 
    }

  return (
    <>
        <div className='RenderTarget'>
            <canvas id="RENDERTARGET"></canvas>
        </div>
        {draw()}
        {bottomBar()}
    </>
  )
}
