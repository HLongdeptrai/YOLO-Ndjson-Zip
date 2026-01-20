import { useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import type { Format } from "@/lib/types";

export function useConverter() {
  const [selectedFile, setSelectedFile] = useState<string | null>(null);
  const [selectedFileName, setSelectedFileName] = useState<string | null>(null);
  const [selectedFormat, setSelectedFormat] = useState<Format | null>(null);

  const selectFile = async () => {
    const file = await open({
      multiple: false,
      filters: [
        { name: "NDJSON", extensions: ["ndjson", "jsonl"] },
        { name: "All Files", extensions: ["*"] },
      ],
    });

    if (file) {
      setSelectedFile(file);
      setSelectedFileName(file.split(/[/\\]/).pop() || file);
    }
  };

  const removeFile = () => {
    setSelectedFile(null);
    setSelectedFileName(null);
    setSelectedFormat(null);
  };

  return {
    selectedFile,
    selectedFileName,
    selectedFormat,
    setSelectedFormat,
    selectFile,
    removeFile,
  };
}
