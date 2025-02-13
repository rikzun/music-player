import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen, Event } from "@tauri-apps/api/event";
import "./App.css";

function App() {
    const [messages, setMessages] = useState<Event<string>[]>([]);
    const [errors, setErrors] = useState<Event<string>[]>([]);
    const [name, setName] = useState("");

    useEffect(() => {
        listen<string>("command", (data) => {
            setMessages((v) => ([...v, data]))
        })

        listen<string>("stdout-event", (data) => {
            setMessages((v) => ([...v, data]))
        })

        listen<string>("stderr-event", (data) => {
            setErrors((v) => ([...v, data]))
        })
    }, [])


    async function greet() {
        setMessages([])
        setErrors([])

        invoke<string>("greet", {value: name})
    }

    return (
        <main className="container">
            <form
                className="row"
                onSubmit={(e) => {
                    e.preventDefault();
                    greet();
                }}
            >
                <input
                    id="greet-input"
                    onChange={(e) => setName(e.currentTarget.value)}
                    placeholder="Enter a name..."
                    spellCheck="false"
                    autoComplete="false"
                />
                <button type="submit">download</button>
            </form>
            <div className="messages">
                {[...messages, ...errors].map((e, i) => (
                    <span
                        key={e.event + e.id + i}
                        style={{textAlign: 'left'}}
                        children={e.payload}
                    />
                ))}
            </div>
        </main>
    );
}

export default App;
