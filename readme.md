## Symmetric Encryption Demonstration 
#### TODO

Pick a language to write a small program in, pick an encryption library available in that language, accept input from a the console or GUI, encrypt the user provided data, save the encrypted results to a file, print the encryption results, decrypt the file, print the decryption results to the console, once your satisfied with your program upload your program to github.com, provide a readme that describes your program and which encryption algorithm you used and why.

#### Why
Publicly demonstrates experience using a 3rd party library and how to encrypt/decrypt data without leaking secrets. 

#### Details
- Create a program that will accept input
- Validate the input on length
- The program must encrypt the given data and save it to disk
- The encryption algorithm must not have any known weaknesses
- The encryption algorithm must be symmetric
- The program must prove the data is encrypted
    - An easy way; read and print the file contents to the console
- Decrypt the file and print the result to the console
- Push your demonstration program to github.com once complete
- Do not leak any secret information
- Provide a short description of the program and the encryption algorithm you used
- Upload a screen capture of your repository to this dropbox

#### Requirements
- Program accepts input
- Program validates input
- Program encrypts input
- Program proves the input was encrypted successfully
- Program saves encrypted input to disk
- Program reads the encrypted file input to disk
- Program decrypts a file on the disk
- Program proves the data was been decrypted successfully
- Key not leaked
- Program is available on github.com
- Readme.md adequately describes program and algorithm.

___

```shell
Symmetric key encryption on the command line, via AES-GCM.

$ post_it -h

Usage: post_it [OPTIONS]

Options:
  -t, --text <TEXT>      Input text enclosed in double quotes, like "encrypt me".
  -i, --input <INPUT>    Input file to read.
  -d, --debug            
  -h, --help             Print help
  -V, --version          Print version
```

