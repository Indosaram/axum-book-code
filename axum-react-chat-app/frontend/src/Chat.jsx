import { useEffect, useState, useContext } from "react";
import {
  Button,
  Card,
  CardBody,
  Center,
  Flex,
  Input,
  Spacer,
  Stack,
  Text,
} from "@chakra-ui/react";
import { useParams } from "react-router-dom";
import { UserContext } from "./Context";

const Chat = () => {
  const [messages, setMessages] = useState([]);
  const [newMessage, setNewMessage] = useState("");
  const username = useContext(UserContext);

  const roomId = useParams().roomId;
  useEffect(() => {
    console.log("[Chat.jsx] username", username);
    fetch(`${import.meta.env.VITE_BACKEND_URL}/chat?room_id=${roomId}`).then(
      (response) => {
        response.json().then((data) => {
          console.log("[Chat.jsx] data", data);
          setMessages(data);
        });
      }
    );

    const eventSource = new EventSource(
      `${import.meta.env.VITE_BACKEND_URL}/chat/subscribe`
    );

    eventSource.onmessage = (event) => {
      const message = JSON.parse(event.data);
      console.log("message", message);
      setMessages((prevMessages) => [...prevMessages, message]);
    };

    return () => {
      eventSource.close();
    };
  }, [roomId, username]);

  const sendMessage = async () => {
    console.log("sendMessage: ", roomId);
    await fetch(`${import.meta.env.VITE_BACKEND_URL}/chat/send`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        sender: username,
        message: newMessage,
        // parse to integer
        room_id: parseInt(roomId),
      }),
    });

    setNewMessage("");
  };

  return (
    <Stack>
      <Stack>
        {messages.length > 0 ? (
          messages.map((message, index) => {
            const timestamp = new Date(message.timestamp).toLocaleString();
            return (
              <Card
                key={index}
                textAlign={message.sender === username ? "right" : "left"}
              >
                <CardBody>
                  <Text>{timestamp}</Text>
                  <Text>{`${message.sender} : ${message.message}`}</Text>
                </CardBody>
              </Card>
            );
          })
        ) : (
          <Center>
            <Text>No messages yet</Text>
          </Center>
        )}
      </Stack>
      <Spacer />
      <Flex>
        <Input
          type="text"
          value={newMessage}
          onChange={(e) => setNewMessage(e.target.value)}
        />
        <Button onClick={sendMessage}>Send</Button>
      </Flex>
    </Stack>
  );
};

export default Chat;
