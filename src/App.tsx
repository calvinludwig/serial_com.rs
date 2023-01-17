import { useEffect, useState } from 'react';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api';

export function App() {
    const [commands, setCommands] = useState<string[]>(['Calvin']);

    useEffect(() => {
        invoke('start_com');
        listen('new-command', (event) => {
            setCommands((state) => [...state, event.payload.new_command]);
        });
    }, []);

    return (
        <div className="bg-neutral-900 text-neutral-50 p-2 h-screen w-screen">
            <ul>
                {commands.map((command, i) => (
                    <p key={i}>{command}</p>
                ))}
            </ul>
        </div>
    );
}
