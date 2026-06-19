# Note Tecniche 

## Scelte Architetturali 

### sqlx vs. SeaORM
Dato il tempo ristretto per la consegna , in vista della fine dello stage ho scelto un approccio più semplice che mettesse in evidenza il funzionamento del tooling layer. Ho quindi scelto di utilizzare uno schema con tabella singola per gli item di una lista della spesa al quale l'agente può aggiungere , leggere , modificare o eliminare item , a seconda della richiesta dell'utente . La scelta è quindi ricaduta su sqlx invece di SeaORM.

### Struttura dei file 
-main.rs > gestisce il loop delle conversazioni 
-openrouter.rs > struttura request/response , chiamata HTTP
-tools.rs > definizioni dei tool, dispatch, query DB

Ho scelto di mettere le funzioni DB nel tools.rs, perchè ogni funzione esiste solamente per funzionare con un tool specifico.

### Nessuna UI
Per minimizzare il carico, ho scelto di non aggiungere server HTTP e frontend. Così ho scelto un input da terminale tramite stdin, permettendomi di testare messaggi senza aggiungere dipendenze . 

### Creazione della tabella al boot
Per via sempre del discorso di "cercare di minimizzare il grasso" la tabella items viene creata in main.rs con "CREATE TABLE IF NOT EXISTS" all'avvio invece di usare un sistema di migrazioni, senza dover aggiungere dipendenze . 

## Cambiamenti sul percorso

### Campo "type" in ToolCall
Durante il test mi sono accorto che OpenRouter richiede "type":"function" nella tool call quando viene reinserita nella history  per la seconda chiamata. La struct iniziale non lo includeva così l'ho aggiunto con #[serde(rename = "type")], perché type è una parola riservata in Rust.

### skip_serializing_if in Message
I campi tool_calls, tool_call_id e content sono opzionali a seconda del ruolo del messaggio. Senza 'skip_serializing_if' serde li serializzerebbe come null ed alcuni provider rifiutano campi null non previsti. Con questo attributo vengono omessi dal JSON se None.

### Loop al posto di round singolo 
La mia prima implementazione gestiva un singolo round di tool calls, il che portava ad un errore. Durante il test del delete , il modello ha chiamato read_list per prendere l'id , poi delete_item, facendo due chiamate sequenziali di tool, senza però rispondere.Il codice non gestiva questo caso e quando arrivava la seconda tool call, tentava di stampare il campo content che era null, e il programma terminava senza eseguire nulla. Risolto sostituendo il blocco if/else con un loop che continua finché il modello non risponde con testo invece di un'altra tool call.
