# Configuración de Diesel ORM

Para utilizar **Diesel** como ORM y configurar las tablas correctamente cuando se inicia un proyecto con este ORM, se utiliza la siguiente secuencia de comandos:

1. **`diesel setup`**  
   - Crea la carpeta llamada `migraciones`.  
   - Genera un archivo llamado `schema.rs` donde se agregarán las tablas.

2. **`diesel migration generate "nombre_de_la_carpeta"`**  
   - Crea una carpeta dentro del directorio de  `migraciones` donde esta la nueva migración.

3. **Editar el archivo `up.sql`**  
   - Escribe el SQL necesario para la creación de las tablas.

4. **Editar el archivo `down.sql`**  
   - Escribe el código SQL necesario para revertir la migración (opcional).

5. **Ejecutar la migración**  
   - Usa el comando:  
     ```bash
     diesel migration run
     ```
   - Esto completa la migración y crea las tablas en la base de datos.
