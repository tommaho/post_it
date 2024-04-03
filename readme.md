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


How I think I want to interact with this:

encrypt text to stdio:

`$ post_it "super secret"`


encrypt text into encrypted file:

`$ post_it "super secret" secret.enc`


encrypt raw input file to stdio:

`$ post_it raw.txt`


read encrypted text to file.

`$ post_it raw.txt secret.enc`

... and all the operations in reverse, decrypting to the various outputs. 

TODO:
* figure out whether file is plaintext or encrypted
    - prefix the encrypted file content with a plaintext `post_it:`
* provide a way to generate new key file

I think I'll use:
https://docs.rs/aes-gcm/latest/aes_gcm/
