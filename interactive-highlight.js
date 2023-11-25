const highlight_element = (ev) => {
  const element = ev.target;

  action = "";
  switch (ev.type) {
    case "change":
    case "click":
      action = element.classList.contains("highlight") ? "unhighlight" : "highlight";
      break;
    case "mouseover":
    case "focusin":
      action = "focus";
      break;
    case "mouseleave":
    case "focusout":
      action = "unfocus";
      break;
  }

  const x_row = element.getAttribute('data-row');
  const x_col = element.getAttribute('data-col');
  let elementType = "cell";
  let coordinates = [parseInt(x_col), parseInt(x_row)];

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

document.querySelectorAll("[data-row],[data-col]")
  .forEach((item) => {
    item.addEventListener("click", highlight_element)
    item.addEventListener("change", highlight_element)
    item.addEventListener("mouseover", highlight_element)
    item.addEventListener("mouseleave", highlight_element)
  })
