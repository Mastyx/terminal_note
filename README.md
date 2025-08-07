# terminal_note


The terminal_note application is a CLI (Command Line Interface) that uses clap for command management
and ratatui for the interactive user interface in note viewing mode.

Here's a summary of its features and "shortcuts" (which in this case are more like command-line commands
and interactions within the UI):

Command Line Commands (CLI):

These are the main commands you can use when starting the application:

* `new -t <title>`: Creates a new note with the specified title. Example: note new -t "My first note"
* `list`: Lists all existing notes.
* `show -t <title>`: Shows the contents of a specific note. Example: note show -t "My first note"
* `delete -t <title>`: Deletes a specific note. Example: note delete -t "Note to delete"
* `edit -t <title>`: Edits an existing note. Opens the note in your default text editor (nvim, vi,
nano, vim, gedit, in that order of preference). Example: note edit -t "Note to edit"

If you don't specify a command (e.g., note), the application runs the list command by default.

Interactions in `list` mode (when listing notes):

When you run note list (or simply note), you enter an interactive mode where
you can navigate and manage notes:

* `Ctrl-Q`: Exits note viewing mode and returns to the command line.
* `Ctrl-N`: Creates a new note. You will be prompted to enter the title of the new note.
* `Enter`: Edits the currently selected note. The note will open in your default text editor.
* `↑` (Up Arrow): Navigates up in the note list.
* `↓` (Down Arrow): Navigate down in the note list.

Interactions in `show` mode (when viewing a note):

When you run note show -t <title>, you enter a view-only mode for that
note:

* `q`: Exit note view and return to the command line.

In summary, the application is designed to be used primarily via command-line commands,
with some additional interactions when viewing the note list or a single note.

WARNING:
The first time you use it, the application searches for the /data/ folder where all notes will be stored. This will only be created when creating a first note.

Example of use:
1) I go into the folder where I'm studying and need to take notes, e.g., `/MATHEMATICS`
2) I create the first note with the command `note new -t first-note`
3) A system text editor will open.

4) Once saved, the note will be created in the /MATHEMATICS/data/ folder where the note file is located.
5) Now, by typing the 'note' command, we will see the list with the created note.


------------------------------------------------------------------------------------------------------------------------

 L'applicazione terminal_note è una CLI (Command Line Interface) che utilizza clap per la gestione dei
  comandi e ratatui per l'interfaccia utente interattiva nella modalità di visualizzazione delle note.

  Ecco un riassunto delle sue funzionalità e degli "shortcut" (che in questo caso sono più simili a comandi
  da riga di comando e interazioni all'interno della UI):

  Comandi da riga di comando (CLI):

  Questi sono i comandi principali che puoi usare quando avvii l'applicazione: 

   * `new -t <titolo>`: Crea una nuova nota con il titolo specificato. Esempio: note new -t "La mia 
     prima nota"
   * `list`: Elenca tutte le note esistenti.
   * `show -t <titolo>`: Mostra il contenuto di una nota specifica. Esempio: note show -t "La mia 
     prima nota"
   * `delete -t <titolo>`: Cancella una nota specifica. Esempio: note delete -t "Nota da cancellare"
   * `edit -t <titolo>`: Modifica una nota esistente. Apre la nota nell'editor di testo predefinito (nvim, vi,
      nano, vim, gedit, in quest'ordine di preferenza). Esempio: note edit -t "Nota da modificare"

  Se non specifichi alcun comando (es. note), l'applicazione esegue di default il comando list.

  Interazioni nella modalità `list` (quando elenchi le note):

  Quando esegui note list (o semplicemente note), entri in una modalità interattiva dove
  puoi navigare e gestire le note:

   * `Ctrl-Q`: Esci dalla modalità di visualizzazione delle note e torni alla riga di comando.
   * `Ctrl-N`: Crea una nuova nota. Ti verrà chiesto di inserire il titolo della nuova nota.
   * `Enter`: Modifica la nota attualmente selezionata. La nota verrà aperta nel tuo editor di testo
     predefinito.
   * `↑` (Freccia Su): Naviga verso l'alto nell'elenco delle note.
   * `↓` (Freccia Giù): Naviga verso il basso nell'elenco delle note.

  Interazioni nella modalità `show` (quando visualizzi una nota):

  Quando esegui note show -t <titolo>, entri in una modalità di sola visualizzazione per quella
  nota:

   * `q`: Esci dalla visualizzazione della nota e torni alla riga di comando.

  In sintesi, l'applicazione è progettata per essere usata principalmente tramite comandi da riga di
  comando, con alcune interazioni aggiuntive quando si visualizza l'elenco delle note o una singola nota.

ATTENZIONE : 
 al primo utilizzo l'applicazione cerca la carteaa /data/ dove verranno conservate tutte le note. Questa verra creata solo creando una prima nota . 

 Es. di utlizzo 
  1) entro nella cartella dove sto studiando e devo prendere degli appunti es. `/MATEMATICA`
  2) creo la prima nota col comando `note new -t prima-nota`
  3) verrà aperto un editor di testo di sistema.
  4) salvata la nota verrà creata nella cartella /MATEMATIA/data/ dove risiede il file della nota
  5) ora dando il comando 'note' vedremo la lista con la nota creata 
