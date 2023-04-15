import React, { useState } from 'react';
import './File.css';

// File can recursively have more files.
// Files can also have content.
function File(props) {
    //props : fileName
    //      : children
    //      : content
    //      : handleClick
   const processClick = () => {
        props.handleClick(props.content);
   }

   const drawChildren= () => {
        return (
            <>
                {props.children.map((element, i, arr) =>
                    <File key={i} handleClick = {props.handleClick} fileName={ i+1 == arr.length ? "└─" +element.fileName : "├─"+element.fileName} content={element.content} children={element.children}></File>
                )}
            </>
        )
        
   }

  return (
    <>
        <div className='file' onClick={() => {processClick()}}>{props.fileName}</div>
        {drawChildren()}
    </>
  )
}

export default File