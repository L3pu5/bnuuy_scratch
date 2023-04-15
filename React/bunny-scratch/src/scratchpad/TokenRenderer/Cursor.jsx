// import { useState, useRef, useLayoutEffect, useEffect } from 'react';
// import React from 'react'
// import './TokenRenderer.css'

// export default function Cursor(props) {
//     var x = document.getElementById("IDSTRING")
//     var height = x?.clientHeight;
//     var canvas = document.getElementById("TOKENTARGET")
//     var width = canvas?.clientWidth;
//     var maxChars = Math.floor(width/x?.clientWidth) + 3;
//     var posYOverFlow = Math.floor(props.col / maxChars)
//     var posX = (props.col % maxChars).toString() + 'ch';
//     //Calculate how many lines above us have wrapped.
//     var lines = props.rawText.split('\n');
//     var cols = [];
//     var wrappedlines = 0;
//     for (let index = 0; index < props.line; index++) {
//         const element = lines[index];
//         cols[index] = element.length;
//         wrappedlines += Math.floor(element.length / maxChars)
//     }
//     var posY = ((props.line + wrappedlines +posYOverFlow) * height).toString() + 'px';
//     //console.log(x?.clientHeight)
//     //height = x.clientHeight;

//   return (
//     <div id="IDSTRING" className='Cursor' style={{left: posX, top: posY }}> </div>
//   )
// }
