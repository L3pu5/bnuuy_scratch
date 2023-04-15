import React from 'react'
import './PsudoDir.css';
import File from './File/File.jsx';

export default function PsudoDir(props) {
    const defaultContent = "This is the default content of a bunnyscratch file.\nPlease feel free to play around with it!\n --- Love, L3Pu5 Hare";
    const rootChildren = [
        {fileName: "temp", content: "This is the temp directory. This will not be saved.", children: []},
    ]
  return (
    <div className="psudoDir">
        <File handleClick={props.handleClick} fileName="/" content={defaultContent} children={rootChildren}></File>
    </div>
  )
}
