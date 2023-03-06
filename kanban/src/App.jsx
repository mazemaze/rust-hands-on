import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import Board from '@asseinfo/react-kanban';
import '@asseinfo/react-kanban/dist/styles.css';
import { invoke } from "@tauri-apps/api";

async function handleAddCard(board, column, card) {
  const pos = new CardPos(column.id, 0);
  await invoke("handle_add_card", {"card" : card, "pos": pos})
}

const board = {
  columns: [
    {
      id: 0,
      title: "バックログ",
      cards: [
        {
          id: 0,
          title: '看板を追加する',
          description: 'react-kanbanを使用する',
        }
      ]
    },
    {
      id: 1,
      title: "開発中",
      cards: []
    }
  ]
}

function App() {
  
  return (
    <>
    <Board 
    initialBoard={board}
    allowAddCard={{on: "top"}}
    allowRemoveCard
    disableColumnDrag
    onNewCardConfirm={(draftCard) => ({
      id: new Date().getTime(),
      ...draftCard
    })}

    onCardNew={console.log}
    onCardDragEnd={console.log}
    onCardRemove={console.log}
    />
    </>
  )
}

export default App;
