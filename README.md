# terminal_note
 
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
