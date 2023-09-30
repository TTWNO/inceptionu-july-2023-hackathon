//const websocketServer = "ws://localhost:3000/ws";
//
//const highlightSocket = new WebSocket(
//    websocketServer,
//);

const highlight_element = (ev) => {
  const element = ev.target;
  const x_row = element.getAttribute('data-row');
  const x_col = element.getAttribute('data-col');
  let elementType = "cell";
  let coordinates = [parseInt(x_col), parseInt(x_row)];
  let action = "highlight";

  if (element.classList.contains("highlight")) {
     action = "unhighlight"
  }

  if (x_col === null) {
    elementType = "row";
    coordinates = [parseInt(x_row)];
  }
  if (x_row === null) {
    elementType = "column";
    coordinates = [parseInt(x_col)];
  }
  
  console.log(JSON.stringify({position: {type: elementType, data: coordinates}, action: action}))
  highlightSocket.send(JSON.stringify({position: {type: elementType, data: coordinates}, action: action}))
}

const highlight_table_checkbox = (ev) => {
  let action = "highlight"
  let clickedItem = ev.target;
  item = clickedItem.parentElement.parentElement;

  const index = item.id.replace('table-', '');

  highlightSocket.send(JSON.stringify({index: Number(index), action: action}));
	ev.stopPropagation();

	return false;
}

const highlight_table_focus = (ev) => {
  let action = "highlight"
  let clickedItem = ev.target;
  item = clickedItem.parentElement;

  const index = item.id.replace('table-', '');

  highlightSocket.send(JSON.stringify({index: Number(index), action: action}));    
}

const highlight_svg_click = (ev) => {
  const bar = ev.target;
  
  const id = Number(bar.id.replace("bar-", ""))
  highlightSocket.send(JSON.stringify({index: id, action: "highlight"}))
}

document.querySelectorAll('input[type="checkbox"]')
	.forEach((item) => {
		item.addEventListener("click", highlight_table_checkbox)
	})

document.querySelectorAll("[data-row][data-col]")
  .forEach((item) => {
    item.addEventListener("click", highlight_element)
  })

document.querySelectorAll("table td.highlightable")
  .forEach((item) => {
    item.addEventListener("focus", highlight_table_focus)
    item.addEventListener("click", highlight_table_focus)
  })
