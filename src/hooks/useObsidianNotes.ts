import { useEffect, useState, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { NoteMetadata, Note, SaveNoteRequest } from "@/lib/tauri";

export function useObsidianNotes() {
  const [notes, setNotes] = useState<NoteMetadata[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const loadNotes = useCallback(async () => {
    try {
      setIsLoading(true);
      const result = await invoke<NoteMetadata[]>("list_notes");
      setNotes(result);
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to load notes");
    } finally {
      setIsLoading(false);
    }
  }, []);

  useEffect(() => {
    loadNotes();
  }, [loadNotes]);

  const readNote = useCallback(async (id: string): Promise<Note | null> => {
    try {
      return await invoke<Note>("read_note", { id });
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to read note");
      return null;
    }
  }, []);

  const saveNote = useCallback(async (request: SaveNoteRequest): Promise<boolean> => {
    try {
      await invoke("save_note", { request });
      await loadNotes(); // Refresh list
      return true;
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to save note");
      return false;
    }
  }, [loadNotes]);

  return {
    notes,
    isLoading,
    error,
    readNote,
    saveNote,
    refresh: loadNotes,
  };
}
