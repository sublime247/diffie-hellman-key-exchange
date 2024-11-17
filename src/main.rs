
// implementing Diffie-Hellman Encryption Algorithm
// Alice---------Bob
// Alice has a private key a=17
// Bob has a private key  b = 19
// they both agree on a relative large prime number  = primitive value P= 23 and  G=5 avalaible publicly
// to compute alice pubic key 5^a modp
// to compute bob pubic key 5^b modp
// they both exchange thier public key to decrypt message accross the channel
// for alice to decrypt the message  she takes the public key of bob B^a mod p 
//for bob to decrypt the message she takes the public key of alice A^bmodp


fn main() {
    fn gen_pub_key(p: usize, g: usize, pr: usize) -> usize {
        let mut result = 1;
        let g_mod_p = g % p;
        for _ in 0..pr {
            result = (result * g_mod_p) % p;
        }
        return result;
    }


    
     let p = 23;
     let g=5;
     let alice_pr_key = 17;
     let bob_pr_key = 19;
    let alice_pub_key =  gen_pub_key(p, g, alice_pr_key);
    println!("{} is alice_pub_key", alice_pub_key);

   let bob_pub_key = gen_pub_key(p, g, bob_pr_key);
   println!("{} is bob_pub_key", bob_pub_key);

//    they will now encrypt the message each other pub_key
 let bob_message_to_alice = gen_pub_key(p, bob_pub_key, alice_pr_key);
 let alice_message_to_bob = gen_pub_key(p, alice_pub_key, bob_pr_key);

 println!("{}", bob_message_to_alice);
 println!("{}", alice_message_to_bob);

    
}
