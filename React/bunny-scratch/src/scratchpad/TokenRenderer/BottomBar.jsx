import React from 'react'
import './TokenRenderer.css'

export default function BottomBar(props) {
  return (
    <div className="BottomBar">Idx: {props.idx} Line: {props.line} Col: {props.col}</div>
  )
}
