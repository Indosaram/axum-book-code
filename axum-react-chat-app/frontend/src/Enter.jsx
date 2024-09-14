import { useState, useEffect } from "react";

import { Flex, Heading, Input, Button, Stack } from "@chakra-ui/react";
import { useNavigate } from "react-router-dom";

function Enter() {
  const [username, setUsername] = useState("");
  const navigate = useNavigate();

  useEffect(
    () => {
      const storedUsername = window.sessionStorage.getItem("username");
      if (storedUsername) {
        navigate("/rooms");
      }
    },
    [navigate]
  );
  return (
    <Flex justifyContent={"center"}>
      <Stack
        style={{
          margin: "10px",
        }}
      >
        <Heading size="md">Enter your username for this session</Heading>
        <Input
          placeholder="Username"
          value={username}
          onChange={(event) => {
            setUsername(event.target.value);
          }}
        />
        <Button
          colorScheme="blue"
          onClick={() => {
            window.sessionStorage.setItem("username", username);
            navigate("/rooms");
          }}
        >
          Enter
        </Button>
      </Stack>
    </Flex>
  );
}

export default Enter;
