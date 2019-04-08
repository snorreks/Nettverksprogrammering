$(document).ready(function() {
  const wSocket = new WebSocket("ws://localhost:3001");

  wSocket.onerror = error => {
    $("#connection_label").html("Not connected");
  };

  wSocket.onopen = () => {
    $("#connection_label").html("Connected");
  };

  wSocket.onmessage = event => {
    const jsonMessage = JSON.parse(event.data);
    const message = jsonMessage.message;
    const messageElement = "<p >" + message + "</p>";
    $("#message-board").append(messageElement);
  };

  wSocket.onclose = function(message) {
    $("#connection_label").html("Not connected");
  };

  $("#new-message-button").click(function() {
    if (wSocket.readyState == 1) {
      const jsonMessage = {
        message: $("#new-message").val()
      };
      wSocket.send(JSON.stringify(jsonMessage));
      $("#new-message").val("");
    }
  });

  $("#new-message").keypress(function(event) {
    if (event.keyCode == 13 && $(this).val() !== "") {
      $("#new-message-button").click();
    }
  });
});
