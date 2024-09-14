/* eslint-disable react/prop-types */
import { useContext, useEffect, useState } from "react";

import {
  SimpleGrid,
  Card,
  CardBody,
  Stack,
  Heading,
  Text,
  ButtonGroup,
  Button,
  CardFooter,
  Flex,
  Spacer,
  Center,
} from "@chakra-ui/react";
import { Link } from "react-router-dom";
import { UserContext } from "./Context";

function Room({ room, deleteRoom }) {
  return (
    <Card
      minW="sm"
      maxW="sm"
      variant="outline"
      style={{
        border: "1px solid black",
      }}
    >
      <CardBody>
        <Stack mt="6" spacing="3">
          <Flex>
            <Heading size="md">Chat room #{room.id}</Heading>
            <Button
              colorScheme="red"
              ml="auto"
              onClick={() => {
                deleteRoom(room.id);
              }}
            >
              Delete
            </Button>
          </Flex>

          <Flex>
            <Text
              style={{
                marginRight: "10px",
              }}
            >
              Participants
            </Text>
            <Text color="blue.600">{room.participants.length}</Text>
          </Flex>
        </Stack>
      </CardBody>
      <CardFooter>
        <ButtonGroup spacing="2">
          <Link to={`/chat/${room.id}`}>
            <Button variant="solid" colorScheme="blue">
              Join
            </Button>
          </Link>
        </ButtonGroup>
      </CardFooter>
    </Card>
  );
}

function Rooms() {
  const [rooms, setRooms] = useState([]);
  const username = useContext(UserContext);

  function getRoom() {
    fetch(`${import.meta.env.VITE_BACKEND_URL}/room`).then((response) => {
      response.json().then((data) => {
        setRooms(data);
      });
    });
  }
  function deleteRoom(room_id) {
    fetch(`${import.meta.env.VITE_BACKEND_URL}/room?id=${room_id}`, {
      method: "DELETE",
      headers: {
        "Access-Control-Allow-Origin": "*",
      },
    }).then(getRoom);
  }

  function createRoom() {
    fetch(`${import.meta.env.VITE_BACKEND_URL}/room`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        participants: [username],
      }),
    }).then(getRoom);
  }

  useEffect(() => {
    getRoom();
  }, []);

  return (
    <Stack>
      <Flex>
        <Spacer />
        <Button colorScheme="blue" onClick={createRoom}>
          Create new room
        </Button>
      </Flex>
      <SimpleGrid columns={2} spacing={10}>
        {rooms.map((room, index) => (
          <Center key={index}>
            <Room room={room} deleteRoom={deleteRoom} />
          </Center>
        ))}
      </SimpleGrid>
    </Stack>
  );
}

export default Rooms;
