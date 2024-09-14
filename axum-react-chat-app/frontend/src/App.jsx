import { useEffect, useState } from "react";
import { Link, Route, Routes } from "react-router-dom";
import Chat from "./Chat";
import Rooms from "./Rooms";
import Enter from "./Enter";
import { Heading } from "@chakra-ui/react";
import { useNavigate } from "react-router-dom";

import "./App.css";
import { UserContext } from "./Context";

function App() {
  const [username, setUsername] = useState("");
  const navigate = useNavigate();

  useEffect(() => {
    const storedUsername = window.sessionStorage.getItem("username");
    console.log("[App.jsx] storedUsername", storedUsername);
    if (storedUsername) {
      setUsername(storedUsername);
    } else {
      if (window.location.pathname !== "/") {
        navigate("/");
      }
    }
  }, [username, navigate]);

  return (
    <UserContext.Provider value={username}>
      <Link to="/">
        <Heading
          size="lg"
          style={{
            marginBottom: "20px",
          }}
        >
          Axum chat app
        </Heading>
      </Link>
      <Routes>
        <Route path="/" element={<Enter />} />
        <Route path="/chat/:roomId" element={<Chat />} />
        <Route path="/rooms" element={<Rooms />} />
      </Routes>
    </UserContext.Provider>
  );
}

export default App;
