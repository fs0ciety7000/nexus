import { useState } from "react";
import { useObsidianNotes } from "@/hooks/useObsidianNotes";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { FileText, RefreshCw, Clock, FileEdit } from "lucide-react";
import { formatRelativeTime, formatBytes } from "@/lib/utils";
import type { Note } from "@/lib/tauri";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";

export default function ObsidianViewer() {
  const { notes, isLoading, error, readNote, refresh } = useObsidianNotes();
  const [selectedNote, setSelectedNote] = useState<Note | null>(null);
  const [loadingNote, setLoadingNote] = useState(false);

  const handleNoteClick = async (noteId: string) => {
    setLoadingNote(true);
    const note = await readNote(noteId);
    if (note) {
      setSelectedNote(note);
    }
    setLoadingNote(false);
  };

  if (error) {
    return (
      <Card className="border-nexus-danger">
        <CardHeader>
          <CardTitle className="flex items-center gap-2 text-nexus-danger">
            <FileText className="h-5 w-5" />
            Obsidian Vault - Error
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="text-nexus-danger text-sm">{error}</div>
          <Button
            variant="outline"
            size="sm"
            className="mt-4"
            onClick={refresh}
          >
            <RefreshCw className="h-4 w-4 mr-2" />
            Retry
          </Button>
        </CardContent>
      </Card>
    );
  }

  return (
    <Card className="row-span-2 flex flex-col max-h-[800px]">
      <CardHeader className="flex-shrink-0">
        <div className="flex items-center justify-between">
          <CardTitle className="flex items-center gap-2">
            <FileText className="h-5 w-5 text-nexus-accent" />
            Obsidian Vault
          </CardTitle>
          <Button
            variant="ghost"
            size="icon"
            onClick={refresh}
            disabled={isLoading}
          >
            <RefreshCw className={`h-4 w-4 ${isLoading ? "animate-spin" : ""}`} />
          </Button>
        </div>
        {notes.length > 0 && (
          <div className="text-xs text-muted-foreground">
            {notes.length} note{notes.length !== 1 ? "s" : ""}
          </div>
        )}
      </CardHeader>
      <CardContent className="flex-1 overflow-hidden p-0">
        {isLoading ? (
          <div className="flex items-center justify-center h-full text-muted-foreground">
            Loading notes...
          </div>
        ) : notes.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-full text-muted-foreground">
            <FileText className="h-12 w-12 mb-2 opacity-50" />
            <p>No notes found in vault</p>
          </div>
        ) : (
          <div className="grid grid-cols-2 h-full">
            {/* Notes List */}
            <div className="border-r border-nexus-border overflow-y-auto">
              <div className="p-4 space-y-2">
                {notes.map((note) => (
                  <button
                    key={note.id}
                    onClick={() => handleNoteClick(note.id)}
                    className={`w-full text-left p-3 rounded-lg transition-smooth hover:bg-nexus-surface ${
                      selectedNote?.metadata.id === note.id
                        ? "bg-nexus-surface border border-nexus-accent"
                        : "border border-transparent"
                    }`}
                  >
                    <div className="flex items-start justify-between gap-2">
                      <div className="flex-1 min-w-0">
                        <h4 className="font-medium text-sm truncate">{note.title}</h4>
                        <div className="flex items-center gap-2 mt-1 text-xs text-muted-foreground">
                          <Clock className="h-3 w-3" />
                          {formatRelativeTime(note.modified)}
                        </div>
                      </div>
                      <div className="text-xs text-muted-foreground">
                        {formatBytes(note.size)}
                      </div>
                    </div>
                    {note.tags.length > 0 && (
                      <div className="flex flex-wrap gap-1 mt-2">
                        {note.tags.slice(0, 3).map((tag) => (
                          <Badge key={tag} variant="nexus" className="text-xs">
                            {tag}
                          </Badge>
                        ))}
                      </div>
                    )}
                  </button>
                ))}
              </div>
            </div>

            {/* Note Viewer */}
            <div className="overflow-y-auto">
              {loadingNote ? (
                <div className="flex items-center justify-center h-full text-muted-foreground">
                  Loading note...
                </div>
              ) : selectedNote ? (
                <div className="p-6">
                  <div className="mb-4 pb-4 border-b border-nexus-border">
                    <h2 className="text-2xl font-bold mb-2">
                      {selectedNote.metadata.title}
                    </h2>
                    <div className="flex items-center gap-4 text-xs text-muted-foreground">
                      <div className="flex items-center gap-1">
                        <Clock className="h-3 w-3" />
                        {formatRelativeTime(selectedNote.metadata.modified)}
                      </div>
                      <div>{formatBytes(selectedNote.metadata.size)}</div>
                    </div>
                  </div>
                  <div className="prose prose-invert prose-sm max-w-none">
                    <ReactMarkdown remarkPlugins={[remarkGfm]}>
                      {selectedNote.content}
                    </ReactMarkdown>
                  </div>
                </div>
              ) : (
                <div className="flex flex-col items-center justify-center h-full text-muted-foreground">
                  <FileEdit className="h-12 w-12 mb-2 opacity-50" />
                  <p>Select a note to view</p>
                </div>
              )}
            </div>
          </div>
        )}
      </CardContent>
    </Card>
  );
}
