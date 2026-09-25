Disassembly of section .text:

00000000001af920 <emuella_j2k_tier1::cleanup_pass_encode::<false>>:
  1af920:	55                                              	push   %rbp
  1af921:	41 57                                           	push   %r15
  1af923:	41 56                                           	push   %r14
  1af925:	41 55                                           	push   %r13
  1af927:	41 54                                           	push   %r12
  1af929:	53                                              	push   %rbx
  1af92a:	48 81 ec c8 00 00 00                            	sub    $0xc8,%rsp
  1af931:	48 89 7c 24 78                                  	mov    %rdi,0x78(%rsp)
  1af936:	48 89 74 24 50                                  	mov    %rsi,0x50(%rsp)
  1af93b:	48 8b 46 40                                     	mov    0x40(%rsi),%rax
  1af93f:	48 89 44 24 60                                  	mov    %rax,0x60(%rsp)
  1af944:	48 85 c0                                        	test   %rax,%rax
  1af947:	0f 84 a0 11 00 00                               	je     1b0aed <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x11cd>
  1af94d:	48 89 d3                                        	mov    %rdx,%rbx
  1af950:	48 8b 44 24 50                                  	mov    0x50(%rsp),%rax
  1af955:	48 8b 50 30                                     	mov    0x30(%rax),%rdx
  1af959:	0f b6 48 48                                     	movzbl 0x48(%rax),%ecx
  1af95d:	83 e1 1f                                        	and    $0x1f,%ecx
  1af960:	b8 01 00 00 00                                  	mov    $0x1,%eax
  1af965:	89 4c 24 5c                                     	mov    %ecx,0x5c(%rsp)
  1af969:	d3 e0                                           	shl    %cl,%eax
  1af96b:	89 44 24 34                                     	mov    %eax,0x34(%rsp)
  1af96f:	48 89 94 24 98 00 00 00                         	mov    %rdx,0x98(%rsp)
  1af977:	48 85 d2                                        	test   %rdx,%rdx
  1af97a:	0f 84 6d 11 00 00                               	je     1b0aed <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x11cd>
  1af980:	48 8b 4c 24 50                                  	mov    0x50(%rsp),%rcx
  1af985:	0f b6 41 49                                     	movzbl 0x49(%rcx),%eax
  1af989:	48 8d 04 c0                                     	lea    (%rax,%rax,8),%rax
  1af98d:	48 8d 04 80                                     	lea    (%rax,%rax,4),%rax
  1af991:	48 8d 15 2c 26 e7 ff                            	lea    -0x18d9d4(%rip),%rdx        # 21fc4 <anon.bd84da5dcd26c1be380e6d835091d5c8.1255.llvm.4847318650076708598+0x4724>
  1af998:	48 01 c2                                        	add    %rax,%rdx
  1af99b:	48 89 54 24 70                                  	mov    %rdx,0x70(%rsp)
  1af9a0:	48 8b 41 38                                     	mov    0x38(%rcx),%rax
  1af9a4:	48 89 44 24 18                                  	mov    %rax,0x18(%rsp)
  1af9a9:	48 8b 39                                        	mov    (%rcx),%rdi
  1af9ac:	4c 8b 61 08                                     	mov    0x8(%rcx),%r12
  1af9b0:	48 8b 41 28                                     	mov    0x28(%rcx),%rax
  1af9b4:	48 89 44 24 28                                  	mov    %rax,0x28(%rsp)
  1af9b9:	48 8b 41 20                                     	mov    0x20(%rcx),%rax
  1af9bd:	48 89 44 24 40                                  	mov    %rax,0x40(%rsp)
  1af9c2:	48 8b 4c 24 60                                  	mov    0x60(%rsp),%rcx
  1af9c7:	48 89 ca                                        	mov    %rcx,%rdx
  1af9ca:	48 c1 ea 02                                     	shr    $0x2,%rdx
  1af9ce:	89 c8                                           	mov    %ecx,%eax
  1af9d0:	83 e0 03                                        	and    $0x3,%eax
  1af9d3:	48 83 f8 01                                     	cmp    $0x1,%rax
  1af9d7:	48 83 da ff                                     	sbb    $0xffffffffffffffff,%rdx
  1af9db:	45 31 ed                                        	xor    %r13d,%r13d
  1af9de:	48 89 5c 24 48                                  	mov    %rbx,0x48(%rsp)
  1af9e3:	4c 89 64 24 68                                  	mov    %r12,0x68(%rsp)
  1af9e8:	48 89 7c 24 08                                  	mov    %rdi,0x8(%rsp)
  1af9ed:	eb 26                                           	jmp    1afa15 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xf5>
  1af9ef:	90                                              	nop
  1af9f0:	48 8b 8c 24 88 00 00 00                         	mov    0x88(%rsp),%rcx
  1af9f8:	48 83 c1 fc                                     	add    $0xfffffffffffffffc,%rcx
  1af9fc:	4c 8b ac 24 80 00 00 00                         	mov    0x80(%rsp),%r13
  1afa04:	48 8b 94 24 90 00 00 00                         	mov    0x90(%rsp),%rdx
  1afa0c:	48 85 d2                                        	test   %rdx,%rdx
  1afa0f:	0f 84 d8 10 00 00                               	je     1b0aed <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x11cd>
  1afa15:	48 83 f9 01                                     	cmp    $0x1,%rcx
  1afa19:	48 89 8c 24 88 00 00 00                         	mov    %rcx,0x88(%rsp)
  1afa21:	48 83 d1 00                                     	adc    $0x0,%rcx
  1afa25:	48 83 f9 04                                     	cmp    $0x4,%rcx
  1afa29:	b8 04 00 00 00                                  	mov    $0x4,%eax
  1afa2e:	48 0f 43 c8                                     	cmovae %rax,%rcx
  1afa32:	48 89 8c 24 c0 00 00 00                         	mov    %rcx,0xc0(%rsp)
  1afa3a:	49 8d 45 04                                     	lea    0x4(%r13),%rax
  1afa3e:	48 89 84 24 80 00 00 00                         	mov    %rax,0x80(%rsp)
  1afa46:	48 ff ca                                        	dec    %rdx
  1afa49:	48 89 94 24 90 00 00 00                         	mov    %rdx,0x90(%rsp)
  1afa51:	48 8b 44 24 60                                  	mov    0x60(%rsp),%rax
  1afa56:	4c 29 e8                                        	sub    %r13,%rax
  1afa59:	48 89 84 24 b0 00 00 00                         	mov    %rax,0xb0(%rsp)
  1afa61:	4c 89 e8                                        	mov    %r13,%rax
  1afa64:	48 83 c8 01                                     	or     $0x1,%rax
  1afa68:	48 0f af 44 24 18                               	imul   0x18(%rsp),%rax
  1afa6e:	48 ff c0                                        	inc    %rax
  1afa71:	48 89 84 24 a8 00 00 00                         	mov    %rax,0xa8(%rsp)
  1afa79:	45 31 f6                                        	xor    %r14d,%r14d
  1afa7c:	4c 89 ac 24 a0 00 00 00                         	mov    %r13,0xa0(%rsp)
  1afa84:	eb 2b                                           	jmp    1afab1 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x191>
  1afa86:	01 43 14                                        	add    %eax,0x14(%rbx)
  1afa89:	48 8b 7c 24 08                                  	mov    0x8(%rsp),%rdi
  1afa8e:	66 90                                           	xchg   %ax,%ax
  1afa90:	4c 8b b4 24 b8 00 00 00                         	mov    0xb8(%rsp),%r14
  1afa98:	49 ff c6                                        	inc    %r14
  1afa9b:	4c 3b b4 24 98 00 00 00                         	cmp    0x98(%rsp),%r14
  1afaa3:	4c 8b ac 24 a0 00 00 00                         	mov    0xa0(%rsp),%r13
  1afaab:	0f 84 3f ff ff ff                               	je     1af9f0 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xd0>
  1afab1:	48 8b 84 24 a8 00 00 00                         	mov    0xa8(%rsp),%rax
  1afab9:	4e 8d 04 30                                     	lea    (%rax,%r14,1),%r8
  1afabd:	48 83 bc 24 b0 00 00 00 04                      	cmpq   $0x4,0xb0(%rsp)
  1afac6:	4c 89 b4 24 b8 00 00 00                         	mov    %r14,0xb8(%rsp)
  1aface:	0f 82 5c 02 00 00                               	jb     1afd30 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x410>
  1afad4:	4d 39 e0                                        	cmp    %r12,%r8
  1afad7:	0f 83 79 10 00 00                               	jae    1b0b56 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x1236>
  1afadd:	4b 8d 04 40                                     	lea    (%r8,%r8,2),%rax
  1afae1:	80 3c 07 00                                     	cmpb   $0x0,(%rdi,%rax,1)
  1afae5:	0f 85 45 02 00 00                               	jne    1afd30 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x410>
  1afaeb:	48 01 f8                                        	add    %rdi,%rax
  1afaee:	80 78 01 00                                     	cmpb   $0x0,0x1(%rax)
  1afaf2:	0f 85 38 02 00 00                               	jne    1afd30 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x410>
  1afaf8:	4c 89 e6                                        	mov    %r12,%rsi
  1afafb:	4c 89 c5                                        	mov    %r8,%rbp
  1afafe:	4c 89 c2                                        	mov    %r8,%rdx
  1afb01:	48 8b 4c 24 18                                  	mov    0x18(%rsp),%rcx
  1afb06:	e8 c5 f7 ff ff                                  	call   1af2d0 <emuella_j2k_tier1::neighborhood_at::<false>>
  1afb0b:	48 8b 7c 24 08                                  	mov    0x8(%rsp),%rdi
  1afb10:	89 c1                                           	mov    %eax,%ecx
  1afb12:	c1 e9 18                                        	shr    $0x18,%ecx
  1afb15:	80 e1 01                                        	and    $0x1,%cl
  1afb18:	48 89 c2                                        	mov    %rax,%rdx
  1afb1b:	48 c1 ea 20                                     	shr    $0x20,%rdx
  1afb1f:	80 e2 01                                        	and    $0x1,%dl
  1afb22:	48 0f ba e0 30                                  	bt     $0x30,%rax
  1afb27:	10 ca                                           	adc    %cl,%dl
  1afb29:	0f ba e0 08                                     	bt     $0x8,%eax
  1afb2d:	80 d2 00                                        	adc    $0x0,%dl
  1afb30:	89 c1                                           	mov    %eax,%ecx
  1afb32:	80 e1 01                                        	and    $0x1,%cl
  1afb35:	89 c6                                           	mov    %eax,%esi
  1afb37:	c1 ee 10                                        	shr    $0x10,%esi
  1afb3a:	40 80 e6 01                                     	and    $0x1,%sil
  1afb3e:	49 b8 ff ff ff ff ff ff ff 00                   	movabs $0xffffffffffffff,%r8
  1afb48:	49 39 c0                                        	cmp    %rax,%r8
  1afb4b:	80 d1 00                                        	adc    $0x0,%cl
  1afb4e:	48 0f ba e0 28                                  	bt     $0x28,%rax
  1afb53:	40 10 f1                                        	adc    %sil,%cl
  1afb56:	08 d1                                           	or     %dl,%cl
  1afb58:	49 89 e8                                        	mov    %rbp,%r8
  1afb5b:	0f 85 cf 01 00 00                               	jne    1afd30 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x410>
  1afb61:	48 8b 44 24 18                                  	mov    0x18(%rsp),%rax
  1afb66:	49 8d 14 00                                     	lea    (%r8,%rax,1),%rdx
  1afb6a:	4c 39 e2                                        	cmp    %r12,%rdx
  1afb6d:	0f 83 f6 0f 00 00                               	jae    1b0b69 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x1249>
  1afb73:	48 8d 04 52                                     	lea    (%rdx,%rdx,2),%rax
  1afb77:	80 3c 07 00                                     	cmpb   $0x0,(%rdi,%rax,1)
  1afb7b:	0f 85 af 01 00 00                               	jne    1afd30 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x410>
  1afb81:	48 01 f8                                        	add    %rdi,%rax
  1afb84:	80 78 01 00                                     	cmpb   $0x0,0x1(%rax)
  1afb88:	0f 85 a2 01 00 00                               	jne    1afd30 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x410>
  1afb8e:	4c 89 e6                                        	mov    %r12,%rsi
  1afb91:	48 8b 4c 24 18                                  	mov    0x18(%rsp),%rcx
  1afb96:	48 89 14 24                                     	mov    %rdx,(%rsp)
  1afb9a:	e8 31 f7 ff ff                                  	call   1af2d0 <emuella_j2k_tier1::neighborhood_at::<false>>
  1afb9f:	48 8b 7c 24 08                                  	mov    0x8(%rsp),%rdi
  1afba4:	89 c1                                           	mov    %eax,%ecx
  1afba6:	c1 e9 18                                        	shr    $0x18,%ecx
  1afba9:	80 e1 01                                        	and    $0x1,%cl
  1afbac:	48 89 c2                                        	mov    %rax,%rdx
  1afbaf:	48 c1 ea 20                                     	shr    $0x20,%rdx
  1afbb3:	80 e2 01                                        	and    $0x1,%dl
  1afbb6:	48 0f ba e0 30                                  	bt     $0x30,%rax
  1afbbb:	10 ca                                           	adc    %cl,%dl
  1afbbd:	0f ba e0 08                                     	bt     $0x8,%eax
  1afbc1:	80 d2 00                                        	adc    $0x0,%dl
  1afbc4:	89 c1                                           	mov    %eax,%ecx
  1afbc6:	80 e1 01                                        	and    $0x1,%cl
  1afbc9:	89 c6                                           	mov    %eax,%esi
  1afbcb:	c1 ee 10                                        	shr    $0x10,%esi
  1afbce:	40 80 e6 01                                     	and    $0x1,%sil
  1afbd2:	49 b8 ff ff ff ff ff ff ff 00                   	movabs $0xffffffffffffff,%r8
  1afbdc:	49 39 c0                                        	cmp    %rax,%r8
  1afbdf:	4c 8b 0c 24                                     	mov    (%rsp),%r9
  1afbe3:	80 d1 00                                        	adc    $0x0,%cl
  1afbe6:	48 0f ba e0 28                                  	bt     $0x28,%rax
  1afbeb:	40 10 f1                                        	adc    %sil,%cl
  1afbee:	08 d1                                           	or     %dl,%cl
  1afbf0:	49 89 e8                                        	mov    %rbp,%r8
  1afbf3:	0f 85 37 01 00 00                               	jne    1afd30 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x410>
  1afbf9:	48 8b 44 24 18                                  	mov    0x18(%rsp),%rax
  1afbfe:	4d 8d 3c 01                                     	lea    (%r9,%rax,1),%r15
  1afc02:	4d 39 e7                                        	cmp    %r12,%r15
  1afc05:	0f 83 71 0f 00 00                               	jae    1b0b7c <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x125c>
  1afc0b:	4b 8d 04 7f                                     	lea    (%r15,%r15,2),%rax
  1afc0f:	80 3c 07 00                                     	cmpb   $0x0,(%rdi,%rax,1)
  1afc13:	0f 85 17 01 00 00                               	jne    1afd30 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x410>
  1afc19:	48 01 f8                                        	add    %rdi,%rax
  1afc1c:	80 78 01 00                                     	cmpb   $0x0,0x1(%rax)
  1afc20:	0f 85 0a 01 00 00                               	jne    1afd30 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x410>
  1afc26:	4c 89 e6                                        	mov    %r12,%rsi
  1afc29:	4c 89 fa                                        	mov    %r15,%rdx
  1afc2c:	48 8b 4c 24 18                                  	mov    0x18(%rsp),%rcx
  1afc31:	e8 9a f6 ff ff                                  	call   1af2d0 <emuella_j2k_tier1::neighborhood_at::<false>>
  1afc36:	48 8b 7c 24 08                                  	mov    0x8(%rsp),%rdi
  1afc3b:	89 c1                                           	mov    %eax,%ecx
  1afc3d:	c1 e9 18                                        	shr    $0x18,%ecx
  1afc40:	80 e1 01                                        	and    $0x1,%cl
  1afc43:	48 89 c2                                        	mov    %rax,%rdx
  1afc46:	48 c1 ea 20                                     	shr    $0x20,%rdx
  1afc4a:	80 e2 01                                        	and    $0x1,%dl
  1afc4d:	48 0f ba e0 30                                  	bt     $0x30,%rax
  1afc52:	10 ca                                           	adc    %cl,%dl
  1afc54:	0f ba e0 08                                     	bt     $0x8,%eax
  1afc58:	80 d2 00                                        	adc    $0x0,%dl
  1afc5b:	89 c1                                           	mov    %eax,%ecx
  1afc5d:	80 e1 01                                        	and    $0x1,%cl
  1afc60:	89 c6                                           	mov    %eax,%esi
  1afc62:	c1 ee 10                                        	shr    $0x10,%esi
  1afc65:	40 80 e6 01                                     	and    $0x1,%sil
  1afc69:	49 b8 ff ff ff ff ff ff ff 00                   	movabs $0xffffffffffffff,%r8
  1afc73:	49 39 c0                                        	cmp    %rax,%r8
  1afc76:	80 d1 00                                        	adc    $0x0,%cl
  1afc79:	48 0f ba e0 28                                  	bt     $0x28,%rax
  1afc7e:	40 10 f1                                        	adc    %sil,%cl
  1afc81:	08 d1                                           	or     %dl,%cl
  1afc83:	49 89 e8                                        	mov    %rbp,%r8
  1afc86:	0f 85 a4 00 00 00                               	jne    1afd30 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x410>
  1afc8c:	48 8b 44 24 18                                  	mov    0x18(%rsp),%rax
  1afc91:	49 8d 0c 07                                     	lea    (%r15,%rax,1),%rcx
  1afc95:	4c 39 e1                                        	cmp    %r12,%rcx
  1afc98:	0f 83 04 0f 00 00                               	jae    1b0ba2 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x1282>
  1afc9e:	48 8d 04 49                                     	lea    (%rcx,%rcx,2),%rax
  1afca2:	80 3c 07 00                                     	cmpb   $0x0,(%rdi,%rax,1)
  1afca6:	0f 85 84 00 00 00                               	jne    1afd30 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x410>
  1afcac:	48 01 f8                                        	add    %rdi,%rax
  1afcaf:	80 78 01 00                                     	cmpb   $0x0,0x1(%rax)
  1afcb3:	75 7b                                           	jne    1afd30 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x410>
  1afcb5:	4c 89 e6                                        	mov    %r12,%rsi
  1afcb8:	48 89 4c 24 38                                  	mov    %rcx,0x38(%rsp)
  1afcbd:	48 8b 54 24 38                                  	mov    0x38(%rsp),%rdx
  1afcc2:	48 8b 4c 24 18                                  	mov    0x18(%rsp),%rcx
  1afcc7:	e8 04 f6 ff ff                                  	call   1af2d0 <emuella_j2k_tier1::neighborhood_at::<false>>
  1afccc:	48 8b 7c 24 08                                  	mov    0x8(%rsp),%rdi
  1afcd1:	89 c1                                           	mov    %eax,%ecx
  1afcd3:	c1 e9 18                                        	shr    $0x18,%ecx
  1afcd6:	80 e1 01                                        	and    $0x1,%cl
  1afcd9:	48 89 c2                                        	mov    %rax,%rdx
  1afcdc:	48 c1 ea 20                                     	shr    $0x20,%rdx
  1afce0:	80 e2 01                                        	and    $0x1,%dl
  1afce3:	48 0f ba e0 30                                  	bt     $0x30,%rax
  1afce8:	10 ca                                           	adc    %cl,%dl
  1afcea:	0f ba e0 08                                     	bt     $0x8,%eax
  1afcee:	80 d2 00                                        	adc    $0x0,%dl
  1afcf1:	89 c1                                           	mov    %eax,%ecx
  1afcf3:	80 e1 01                                        	and    $0x1,%cl
  1afcf6:	89 c6                                           	mov    %eax,%esi
  1afcf8:	c1 ee 10                                        	shr    $0x10,%esi
  1afcfb:	40 80 e6 01                                     	and    $0x1,%sil
  1afcff:	49 b8 ff ff ff ff ff ff ff 00                   	movabs $0xffffffffffffff,%r8
  1afd09:	49 39 c0                                        	cmp    %rax,%r8
  1afd0c:	4c 8b 0c 24                                     	mov    (%rsp),%r9
  1afd10:	80 d1 00                                        	adc    $0x0,%cl
  1afd13:	48 0f ba e0 28                                  	bt     $0x28,%rax
  1afd18:	40 10 f1                                        	adc    %sil,%cl
  1afd1b:	08 d1                                           	or     %dl,%cl
  1afd1d:	48 8b 54 24 38                                  	mov    0x38(%rsp),%rdx
  1afd22:	49 89 e8                                        	mov    %rbp,%r8
  1afd25:	0f 84 0b 03 00 00                               	je     1b0036 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x716>
  1afd2b:	0f 1f 44 00 00                                  	nopl   0x0(%rax,%rax,1)
  1afd30:	4c 39 6c 24 60                                  	cmp    %r13,0x60(%rsp)
  1afd35:	0f 84 55 fd ff ff                               	je     1afa90 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x170>
  1afd3b:	45 31 ff                                        	xor    %r15d,%r15d
  1afd3e:	eb 16                                           	jmp    1afd56 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x436>
  1afd40:	49 ff c7                                        	inc    %r15
  1afd43:	4c 03 44 24 18                                  	add    0x18(%rsp),%r8
  1afd48:	4c 3b bc 24 c0 00 00 00                         	cmp    0xc0(%rsp),%r15
  1afd50:	0f 84 3a fd ff ff                               	je     1afa90 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x170>
  1afd56:	4d 39 e0                                        	cmp    %r12,%r8
  1afd59:	0f 83 a8 0d 00 00                               	jae    1b0b07 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x11e7>
  1afd5f:	4f 8d 34 40                                     	lea    (%r8,%r8,2),%r14
  1afd63:	42 80 3c 37 00                                  	cmpb   $0x0,(%rdi,%r14,1)
  1afd68:	75 d6                                           	jne    1afd40 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x420>
  1afd6a:	49 01 fe                                        	add    %rdi,%r14
  1afd6d:	41 80 7e 01 00                                  	cmpb   $0x0,0x1(%r14)
  1afd72:	75 cc                                           	jne    1afd40 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x420>
  1afd74:	4c 3b 44 24 28                                  	cmp    0x28(%rsp),%r8
  1afd79:	0f 83 c2 0d 00 00                               	jae    1b0b41 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x1221>
  1afd7f:	48 8b 44 24 40                                  	mov    0x40(%rsp),%rax
  1afd84:	42 8b 04 80                                     	mov    (%rax,%r8,4),%eax
  1afd88:	45 31 ed                                        	xor    %r13d,%r13d
  1afd8b:	8b 4c 24 5c                                     	mov    0x5c(%rsp),%ecx
  1afd8f:	0f a3 c8                                        	bt     %ecx,%eax
  1afd92:	40 0f 92 c5                                     	setb   %bpl
  1afd96:	4c 89 e6                                        	mov    %r12,%rsi
  1afd99:	4c 89 04 24                                     	mov    %r8,(%rsp)
  1afd9d:	4c 89 c2                                        	mov    %r8,%rdx
  1afda0:	48 8b 4c 24 18                                  	mov    0x18(%rsp),%rcx
  1afda5:	e8 26 f5 ff ff                                  	call   1af2d0 <emuella_j2k_tier1::neighborhood_at::<false>>
  1afdaa:	89 c1                                           	mov    %eax,%ecx
  1afdac:	c1 e9 18                                        	shr    $0x18,%ecx
  1afdaf:	83 e1 01                                        	and    $0x1,%ecx
  1afdb2:	89 c2                                           	mov    %eax,%edx
  1afdb4:	c1 ea 08                                        	shr    $0x8,%edx
  1afdb7:	83 e2 01                                        	and    $0x1,%edx
  1afdba:	48 0f ba e0 30                                  	bt     $0x30,%rax
  1afdbf:	48 83 d2 00                                     	adc    $0x0,%rdx
  1afdc3:	89 c6                                           	mov    %eax,%esi
  1afdc5:	83 e6 01                                        	and    $0x1,%esi
  1afdc8:	89 c7                                           	mov    %eax,%edi
  1afdca:	c1 ef 10                                        	shr    $0x10,%edi
  1afdcd:	83 e7 01                                        	and    $0x1,%edi
  1afdd0:	49 89 c0                                        	mov    %rax,%r8
  1afdd3:	49 c1 e8 38                                     	shr    $0x38,%r8
  1afdd7:	49 01 f0                                        	add    %rsi,%r8
  1afdda:	48 0f ba e0 28                                  	bt     $0x28,%rax
  1afddf:	49 11 f8                                        	adc    %rdi,%r8
  1afde2:	48 8d 14 52                                     	lea    (%rdx,%rdx,2),%rdx
  1afde6:	48 0f ba e0 20                                  	bt     $0x20,%rax
  1afdeb:	48 13 4c 24 70                                  	adc    0x70(%rsp),%rcx
  1afdf0:	4b 8d 04 c0                                     	lea    (%r8,%r8,8),%rax
  1afdf4:	48 01 d1                                        	add    %rdx,%rcx
  1afdf7:	0f b6 3c 01                                     	movzbl (%rcx,%rax,1),%edi
  1afdfb:	48 83 ff 12                                     	cmp    $0x12,%rdi
  1afdff:	0f 87 15 0d 00 00                               	ja     1b0b1a <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x11fa>
  1afe05:	0f b6 44 7b 1c                                  	movzbl 0x1c(%rbx,%rdi,2),%eax
  1afe0a:	48 83 f8 2e                                     	cmp    $0x2e,%rax
  1afe0e:	0f 87 18 0d 00 00                               	ja     1b0b2c <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x120c>
  1afe14:	41 88 ed                                        	mov    %bpl,%r13b
  1afe17:	0f b6 54 7b 1d                                  	movzbl 0x1d(%rbx,%rdi,2),%edx
  1afe1c:	4c 8d 0d 4d 27 e7 ff                            	lea    -0x18d8b3(%rip),%r9        # 22570 <anon.c3db339937c4b26e029c5c277d8a0513.101.llvm.11782617808337995929+0x31>
  1afe23:	41 8b 0c c1                                     	mov    (%r9,%rax,8),%ecx
  1afe27:	41 0f b6 74 c1 04                               	movzbl 0x4(%r9,%rax,8),%esi
  1afe2d:	45 0f b6 44 c1 05                               	movzbl 0x5(%r9,%rax,8),%r8d
  1afe33:	45 0f b6 4c c1 06                               	movzbl 0x6(%r9,%rax,8),%r9d
  1afe39:	8b 43 10                                        	mov    0x10(%rbx),%eax
  1afe3c:	29 c8                                           	sub    %ecx,%eax
  1afe3e:	89 43 10                                        	mov    %eax,0x10(%rbx)
  1afe41:	41 39 d5                                        	cmp    %edx,%r13d
  1afe44:	44 89 6c 24 20                                  	mov    %r13d,0x20(%rsp)
  1afe49:	75 24                                           	jne    1afe6f <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x54f>
  1afe4b:	66 85 c0                                        	test   %ax,%ax
  1afe4e:	78 41                                           	js     1afe91 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x571>
  1afe50:	39 c8                                           	cmp    %ecx,%eax
  1afe52:	48 8b 54 24 08                                  	mov    0x8(%rsp),%rdx
  1afe57:	73 68                                           	jae    1afec1 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x5a1>
  1afe59:	89 4b 10                                        	mov    %ecx,0x10(%rbx)
  1afe5c:	40 88 74 7b 1c                                  	mov    %sil,0x1c(%rbx,%rdi,2)
  1afe61:	48 89 d7                                        	mov    %rdx,%rdi
  1afe64:	66 85 c9                                        	test   %cx,%cx
  1afe67:	0f 88 93 01 00 00                               	js     1b0000 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x6e0>
  1afe6d:	eb 68                                           	jmp    1afed7 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x5b7>
  1afe6f:	39 c8                                           	cmp    %ecx,%eax
  1afe71:	73 2b                                           	jae    1afe9e <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x57e>
  1afe73:	01 4b 14                                        	add    %ecx,0x14(%rbx)
  1afe76:	89 c1                                           	mov    %eax,%ecx
  1afe78:	44 88 44 7b 1c                                  	mov    %r8b,0x1c(%rbx,%rdi,2)
  1afe7d:	45 84 c9                                        	test   %r9b,%r9b
  1afe80:	75 29                                           	jne    1afeab <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x58b>
  1afe82:	48 8b 7c 24 08                                  	mov    0x8(%rsp),%rdi
  1afe87:	66 85 c9                                        	test   %cx,%cx
  1afe8a:	79 4b                                           	jns    1afed7 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x5b7>
  1afe8c:	e9 6f 01 00 00                                  	jmp    1b0000 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x6e0>
  1afe91:	01 4b 14                                        	add    %ecx,0x14(%rbx)
  1afe94:	48 8b 7c 24 08                                  	mov    0x8(%rsp),%rdi
  1afe99:	e9 62 01 00 00                                  	jmp    1b0000 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x6e0>
  1afe9e:	89 4b 10                                        	mov    %ecx,0x10(%rbx)
  1afea1:	44 88 44 7b 1c                                  	mov    %r8b,0x1c(%rbx,%rdi,2)
  1afea6:	45 84 c9                                        	test   %r9b,%r9b
  1afea9:	74 d7                                           	je     1afe82 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x562>
  1afeab:	80 f2 01                                        	xor    $0x1,%dl
  1afeae:	88 54 7b 1d                                     	mov    %dl,0x1d(%rbx,%rdi,2)
  1afeb2:	48 8b 7c 24 08                                  	mov    0x8(%rsp),%rdi
  1afeb7:	66 85 c9                                        	test   %cx,%cx
  1afeba:	79 1b                                           	jns    1afed7 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x5b7>
  1afebc:	e9 3f 01 00 00                                  	jmp    1b0000 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x6e0>
  1afec1:	01 4b 14                                        	add    %ecx,0x14(%rbx)
  1afec4:	89 c1                                           	mov    %eax,%ecx
  1afec6:	40 88 74 7b 1c                                  	mov    %sil,0x1c(%rbx,%rdi,2)
  1afecb:	48 89 d7                                        	mov    %rdx,%rdi
  1afece:	66 85 c9                                        	test   %cx,%cx
  1afed1:	0f 88 29 01 00 00                               	js     1b0000 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x6e0>
  1afed7:	48 8b 6b 08                                     	mov    0x8(%rbx),%rbp
  1afedb:	0f b6 53 45                                     	movzbl 0x45(%rbx),%edx
  1afedf:	44 0f b6 6b 46                                  	movzbl 0x46(%rbx),%r13d
  1afee4:	44 8b 63 14                                     	mov    0x14(%rbx),%r12d
  1afee8:	8b 43 18                                        	mov    0x18(%rbx),%eax
  1afeeb:	eb 45                                           	jmp    1aff32 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x612>
  1afeed:	48 8b 45 08                                     	mov    0x8(%rbp),%rax
  1afef1:	44 88 2c 18                                     	mov    %r13b,(%rax,%rbx,1)
  1afef5:	48 ff c3                                        	inc    %rbx
  1afef8:	48 89 5d 10                                     	mov    %rbx,0x10(%rbp)
  1afefc:	48 8b 5c 24 48                                  	mov    0x48(%rsp),%rbx
  1aff01:	45 89 e5                                        	mov    %r12d,%r13d
  1aff04:	41 c1 ed 13                                     	shr    $0x13,%r13d
  1aff08:	b8 08 00 00 00                                  	mov    $0x8,%eax
  1aff0d:	b9 fe ff 07 00                                  	mov    $0x7fffe,%ecx
  1aff12:	c6 43 45 01                                     	movb   $0x1,0x45(%rbx)
  1aff16:	44 88 6b 46                                     	mov    %r13b,0x46(%rbx)
  1aff1a:	41 21 cc                                        	and    %ecx,%r12d
  1aff1d:	44 89 63 14                                     	mov    %r12d,0x14(%rbx)
  1aff21:	89 43 18                                        	mov    %eax,0x18(%rbx)
  1aff24:	8b 4b 10                                        	mov    0x10(%rbx),%ecx
  1aff27:	b2 01                                           	mov    $0x1,%dl
  1aff29:	66 85 c9                                        	test   %cx,%cx
  1aff2c:	0f 88 ce 00 00 00                               	js     1b0000 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x6e0>
  1aff32:	01 c9                                           	add    %ecx,%ecx
  1aff34:	89 4b 10                                        	mov    %ecx,0x10(%rbx)
  1aff37:	45 01 e4                                        	add    %r12d,%r12d
  1aff3a:	44 89 63 14                                     	mov    %r12d,0x14(%rbx)
  1aff3e:	ff c8                                           	dec    %eax
  1aff40:	89 43 18                                        	mov    %eax,0x18(%rbx)
  1aff43:	75 e4                                           	jne    1aff29 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x609>
  1aff45:	c6 43 45 00                                     	movb   $0x0,0x45(%rbx)
  1aff49:	80 fa 01                                        	cmp    $0x1,%dl
  1aff4c:	75 b3                                           	jne    1aff01 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x5e1>
  1aff4e:	41 80 fd ff                                     	cmp    $0xff,%r13b
  1aff52:	74 33                                           	je     1aff87 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x667>
  1aff54:	41 81 fc 00 00 00 08                            	cmp    $0x8000000,%r12d
  1aff5b:	72 09                                           	jb     1aff66 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x646>
  1aff5d:	41 fe c5                                        	inc    %r13b
  1aff60:	41 80 fd ff                                     	cmp    $0xff,%r13b
  1aff64:	74 4d                                           	je     1affb3 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x693>
  1aff66:	48 8b 5d 10                                     	mov    0x10(%rbp),%rbx
  1aff6a:	48 3b 5d 00                                     	cmp    0x0(%rbp),%rbx
  1aff6e:	0f 85 79 ff ff ff                               	jne    1afeed <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x5cd>
  1aff74:	48 89 ef                                        	mov    %rbp,%rdi
  1aff77:	ff 15 73 de 0b 00                               	call   *0xbde73(%rip)        # 26ddf0 <_DYNAMIC+0x270>
  1aff7d:	48 8b 7c 24 08                                  	mov    0x8(%rsp),%rdi
  1aff82:	e9 66 ff ff ff                                  	jmp    1afeed <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x5cd>
  1aff87:	48 8b 5d 10                                     	mov    0x10(%rbp),%rbx
  1aff8b:	48 3b 5d 00                                     	cmp    0x0(%rbp),%rbx
  1aff8f:	75 0e                                           	jne    1aff9f <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x67f>
  1aff91:	48 89 ef                                        	mov    %rbp,%rdi
  1aff94:	ff 15 56 de 0b 00                               	call   *0xbde56(%rip)        # 26ddf0 <_DYNAMIC+0x270>
  1aff9a:	48 8b 7c 24 08                                  	mov    0x8(%rsp),%rdi
  1aff9f:	48 8b 45 08                                     	mov    0x8(%rbp),%rax
  1affa3:	c6 04 18 ff                                     	movb   $0xff,(%rax,%rbx,1)
  1affa7:	48 ff c3                                        	inc    %rbx
  1affaa:	48 89 5d 10                                     	mov    %rbx,0x10(%rbp)
  1affae:	45 89 e5                                        	mov    %r12d,%r13d
  1affb1:	eb 35                                           	jmp    1affe8 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x6c8>
  1affb3:	45 89 e5                                        	mov    %r12d,%r13d
  1affb6:	41 81 e5 fe ff ff 07                            	and    $0x7fffffe,%r13d
  1affbd:	44 89 6b 14                                     	mov    %r13d,0x14(%rbx)
  1affc1:	48 8b 5d 10                                     	mov    0x10(%rbp),%rbx
  1affc5:	48 3b 5d 00                                     	cmp    0x0(%rbp),%rbx
  1affc9:	75 0e                                           	jne    1affd9 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x6b9>
  1affcb:	48 89 ef                                        	mov    %rbp,%rdi
  1affce:	ff 15 1c de 0b 00                               	call   *0xbde1c(%rip)        # 26ddf0 <_DYNAMIC+0x270>
  1affd4:	48 8b 7c 24 08                                  	mov    0x8(%rsp),%rdi
  1affd9:	48 8b 45 08                                     	mov    0x8(%rbp),%rax
  1affdd:	c6 04 18 ff                                     	movb   $0xff,(%rax,%rbx,1)
  1affe1:	48 ff c3                                        	inc    %rbx
  1affe4:	48 89 5d 10                                     	mov    %rbx,0x10(%rbp)
  1affe8:	41 c1 ed 14                                     	shr    $0x14,%r13d
  1affec:	b8 07 00 00 00                                  	mov    $0x7,%eax
  1afff1:	b9 fe ff 0f 00                                  	mov    $0xffffe,%ecx
  1afff6:	48 8b 5c 24 48                                  	mov    0x48(%rsp),%rbx
  1afffb:	e9 12 ff ff ff                                  	jmp    1aff12 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x5f2>
  1b0000:	83 7c 24 20 00                                  	cmpl   $0x0,0x20(%rsp)
  1b0005:	4c 8b 64 24 68                                  	mov    0x68(%rsp),%r12
  1b000a:	4c 8b 04 24                                     	mov    (%rsp),%r8
  1b000e:	0f 84 2c fd ff ff                               	je     1afd40 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x420>
  1b0014:	4c 89 c7                                        	mov    %r8,%rdi
  1b0017:	48 8b 74 24 50                                  	mov    0x50(%rsp),%rsi
  1b001c:	48 89 da                                        	mov    %rbx,%rdx
  1b001f:	e8 fc f5 ff ff                                  	call   1af620 <emuella_j2k_tier1::encode_sign_bit_at::<false>>
  1b0024:	4c 8b 04 24                                     	mov    (%rsp),%r8
  1b0028:	48 8b 7c 24 08                                  	mov    0x8(%rsp),%rdi
  1b002d:	41 c6 06 01                                     	movb   $0x1,(%r14)
  1b0031:	e9 0a fd ff ff                                  	jmp    1afd40 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x420>
  1b0036:	4c 3b 44 24 28                                  	cmp    0x28(%rsp),%r8
  1b003b:	0f 83 ce 0b 00 00                               	jae    1b0c0f <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x12ef>
  1b0041:	40 b6 01                                        	mov    $0x1,%sil
  1b0044:	8b 44 24 34                                     	mov    0x34(%rsp),%eax
  1b0048:	48 8b 4c 24 40                                  	mov    0x40(%rsp),%rcx
  1b004d:	42 85 04 81                                     	test   %eax,(%rcx,%r8,4)
  1b0051:	74 0b                                           	je     1b005e <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x73e>
  1b0053:	48 c7 44 24 20 00 00 00 00                      	movq   $0x0,0x20(%rsp)
  1b005c:	eb 71                                           	jmp    1b00cf <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x7af>
  1b005e:	4c 3b 4c 24 28                                  	cmp    0x28(%rsp),%r9
  1b0063:	0f 83 73 0b 00 00                               	jae    1b0bdc <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x12bc>
  1b0069:	b8 01 00 00 00                                  	mov    $0x1,%eax
  1b006e:	48 89 44 24 20                                  	mov    %rax,0x20(%rsp)
  1b0073:	8b 44 24 34                                     	mov    0x34(%rsp),%eax
  1b0077:	48 8b 4c 24 40                                  	mov    0x40(%rsp),%rcx
  1b007c:	42 85 04 89                                     	test   %eax,(%rcx,%r9,4)
  1b0080:	75 4d                                           	jne    1b00cf <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x7af>
  1b0082:	4c 3b 7c 24 28                                  	cmp    0x28(%rsp),%r15
  1b0087:	0f 83 67 0b 00 00                               	jae    1b0bf4 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x12d4>
  1b008d:	b8 02 00 00 00                                  	mov    $0x2,%eax
  1b0092:	48 89 44 24 20                                  	mov    %rax,0x20(%rsp)
  1b0097:	8b 44 24 34                                     	mov    0x34(%rsp),%eax
  1b009b:	48 8b 4c 24 40                                  	mov    0x40(%rsp),%rcx
  1b00a0:	42 85 04 b9                                     	test   %eax,(%rcx,%r15,4)
  1b00a4:	75 29                                           	jne    1b00cf <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x7af>
  1b00a6:	48 3b 54 24 28                                  	cmp    0x28(%rsp),%rdx
  1b00ab:	0f 83 5b 0b 00 00                               	jae    1b0c0c <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x12ec>
  1b00b1:	8b 44 24 34                                     	mov    0x34(%rsp),%eax
  1b00b5:	48 8b 4c 24 40                                  	mov    0x40(%rsp),%rcx
  1b00ba:	85 04 91                                        	test   %eax,(%rcx,%rdx,4)
  1b00bd:	0f 84 6f 08 00 00                               	je     1b0932 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x1012>
  1b00c3:	b8 03 00 00 00                                  	mov    $0x3,%eax
  1b00c8:	48 89 44 24 20                                  	mov    %rax,0x20(%rsp)
  1b00cd:	31 f6                                           	xor    %esi,%esi
  1b00cf:	89 74 24 38                                     	mov    %esi,0x38(%rsp)
  1b00d3:	0f b6 7b 3e                                     	movzbl 0x3e(%rbx),%edi
  1b00d7:	48 83 ff 2e                                     	cmp    $0x2e,%rdi
  1b00db:	0f 87 e9 0a 00 00                               	ja     1b0bca <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x12aa>
  1b00e1:	0f b6 43 3f                                     	movzbl 0x3f(%rbx),%eax
  1b00e5:	48 8d 0d 84 24 e7 ff                            	lea    -0x18db7c(%rip),%rcx        # 22570 <anon.c3db339937c4b26e029c5c277d8a0513.101.llvm.11782617808337995929+0x31>
  1b00ec:	8b 14 f9                                        	mov    (%rcx,%rdi,8),%edx
  1b00ef:	0f b6 74 f9 04                                  	movzbl 0x4(%rcx,%rdi,8),%esi
  1b00f4:	44 0f b6 44 f9 05                               	movzbl 0x5(%rcx,%rdi,8),%r8d
  1b00fa:	0f b6 7c f9 06                                  	movzbl 0x6(%rcx,%rdi,8),%edi
  1b00ff:	8b 4b 10                                        	mov    0x10(%rbx),%ecx
  1b0102:	29 d1                                           	sub    %edx,%ecx
  1b0104:	89 4b 10                                        	mov    %ecx,0x10(%rbx)
  1b0107:	3c 01                                           	cmp    $0x1,%al
  1b0109:	75 0e                                           	jne    1b0119 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x7f9>
  1b010b:	66 85 c9                                        	test   %cx,%cx
  1b010e:	78 14                                           	js     1b0124 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x804>
  1b0110:	39 d1                                           	cmp    %edx,%ecx
  1b0112:	73 2b                                           	jae    1b013f <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x81f>
  1b0114:	89 53 10                                        	mov    %edx,0x10(%rbx)
  1b0117:	eb 2b                                           	jmp    1b0144 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x824>
  1b0119:	39 d1                                           	cmp    %edx,%ecx
  1b011b:	73 0f                                           	jae    1b012c <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x80c>
  1b011d:	01 53 14                                        	add    %edx,0x14(%rbx)
  1b0120:	89 ca                                           	mov    %ecx,%edx
  1b0122:	eb 0b                                           	jmp    1b012f <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x80f>
  1b0124:	01 53 14                                        	add    %edx,0x14(%rbx)
  1b0127:	e9 5d 01 00 00                                  	jmp    1b0289 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x969>
  1b012c:	89 53 10                                        	mov    %edx,0x10(%rbx)
  1b012f:	44 88 43 3e                                     	mov    %r8b,0x3e(%rbx)
  1b0133:	40 84 ff                                        	test   %dil,%dil
  1b0136:	74 10                                           	je     1b0148 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x828>
  1b0138:	34 01                                           	xor    $0x1,%al
  1b013a:	88 43 3f                                        	mov    %al,0x3f(%rbx)
  1b013d:	eb 09                                           	jmp    1b0148 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x828>
  1b013f:	01 53 14                                        	add    %edx,0x14(%rbx)
  1b0142:	89 ca                                           	mov    %ecx,%edx
  1b0144:	40 88 73 3e                                     	mov    %sil,0x3e(%rbx)
  1b0148:	66 85 d2                                        	test   %dx,%dx
  1b014b:	0f 88 36 01 00 00                               	js     1b0287 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x967>
  1b0151:	48 8b 7b 08                                     	mov    0x8(%rbx),%rdi
  1b0155:	0f b6 73 45                                     	movzbl 0x45(%rbx),%esi
  1b0159:	44 0f b6 43 46                                  	movzbl 0x46(%rbx),%r8d
  1b015e:	44 8b 73 14                                     	mov    0x14(%rbx),%r14d
  1b0162:	8b 43 18                                        	mov    0x18(%rbx),%eax
  1b0165:	89 d1                                           	mov    %edx,%ecx
  1b0167:	48 89 3c 24                                     	mov    %rdi,(%rsp)
  1b016b:	eb 41                                           	jmp    1b01ae <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x88e>
  1b016d:	48 8b 47 08                                     	mov    0x8(%rdi),%rax
  1b0171:	46 88 04 38                                     	mov    %r8b,(%rax,%r15,1)
  1b0175:	49 ff c7                                        	inc    %r15
  1b0178:	4c 89 7f 10                                     	mov    %r15,0x10(%rdi)
  1b017c:	45 89 f0                                        	mov    %r14d,%r8d
  1b017f:	41 c1 e8 13                                     	shr    $0x13,%r8d
  1b0183:	b8 08 00 00 00                                  	mov    $0x8,%eax
  1b0188:	b9 fe ff 07 00                                  	mov    $0x7fffe,%ecx
  1b018d:	c6 43 45 01                                     	movb   $0x1,0x45(%rbx)
  1b0191:	44 88 43 46                                     	mov    %r8b,0x46(%rbx)
  1b0195:	41 21 ce                                        	and    %ecx,%r14d
  1b0198:	44 89 73 14                                     	mov    %r14d,0x14(%rbx)
  1b019c:	89 43 18                                        	mov    %eax,0x18(%rbx)
  1b019f:	8b 4b 10                                        	mov    0x10(%rbx),%ecx
  1b01a2:	40 b6 01                                        	mov    $0x1,%sil
  1b01a5:	66 85 c9                                        	test   %cx,%cx
  1b01a8:	0f 88 db 00 00 00                               	js     1b0289 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x969>
  1b01ae:	01 c9                                           	add    %ecx,%ecx
  1b01b0:	89 4b 10                                        	mov    %ecx,0x10(%rbx)
  1b01b3:	45 01 f6                                        	add    %r14d,%r14d
  1b01b6:	44 89 73 14                                     	mov    %r14d,0x14(%rbx)
  1b01ba:	ff c8                                           	dec    %eax
  1b01bc:	89 43 18                                        	mov    %eax,0x18(%rbx)
  1b01bf:	75 e4                                           	jne    1b01a5 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x885>
  1b01c1:	c6 43 45 00                                     	movb   $0x0,0x45(%rbx)
  1b01c5:	40 80 fe 01                                     	cmp    $0x1,%sil
  1b01c9:	75 b1                                           	jne    1b017c <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x85c>
  1b01cb:	41 80 f8 ff                                     	cmp    $0xff,%r8b
  1b01cf:	74 38                                           	je     1b0209 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x8e9>
  1b01d1:	41 81 fe 00 00 00 08                            	cmp    $0x8000000,%r14d
  1b01d8:	72 09                                           	jb     1b01e3 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x8c3>
  1b01da:	41 fe c0                                        	inc    %r8b
  1b01dd:	41 80 f8 ff                                     	cmp    $0xff,%r8b
  1b01e1:	74 52                                           	je     1b0235 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x915>
  1b01e3:	4c 8b 7f 10                                     	mov    0x10(%rdi),%r15
  1b01e7:	4c 3b 3f                                        	cmp    (%rdi),%r15
  1b01ea:	75 81                                           	jne    1b016d <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x84d>
  1b01ec:	48 8b 3c 24                                     	mov    (%rsp),%rdi
  1b01f0:	44 89 44 24 14                                  	mov    %r8d,0x14(%rsp)
  1b01f5:	ff 15 f5 db 0b 00                               	call   *0xbdbf5(%rip)        # 26ddf0 <_DYNAMIC+0x270>
  1b01fb:	44 8b 44 24 14                                  	mov    0x14(%rsp),%r8d
  1b0200:	48 8b 3c 24                                     	mov    (%rsp),%rdi
  1b0204:	e9 64 ff ff ff                                  	jmp    1b016d <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x84d>
  1b0209:	4c 8b 7f 10                                     	mov    0x10(%rdi),%r15
  1b020d:	4c 3b 3f                                        	cmp    (%rdi),%r15
  1b0210:	75 0e                                           	jne    1b0220 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x900>
  1b0212:	48 8b 3c 24                                     	mov    (%rsp),%rdi
  1b0216:	ff 15 d4 db 0b 00                               	call   *0xbdbd4(%rip)        # 26ddf0 <_DYNAMIC+0x270>
  1b021c:	48 8b 3c 24                                     	mov    (%rsp),%rdi
  1b0220:	48 8b 47 08                                     	mov    0x8(%rdi),%rax
  1b0224:	42 c6 04 38 ff                                  	movb   $0xff,(%rax,%r15,1)
  1b0229:	49 ff c7                                        	inc    %r15
  1b022c:	4c 89 7f 10                                     	mov    %r15,0x10(%rdi)
  1b0230:	45 89 f0                                        	mov    %r14d,%r8d
  1b0233:	eb 3f                                           	jmp    1b0274 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x954>
  1b0235:	45 89 f0                                        	mov    %r14d,%r8d
  1b0238:	41 81 e0 fe ff ff 07                            	and    $0x7fffffe,%r8d
  1b023f:	44 89 43 14                                     	mov    %r8d,0x14(%rbx)
  1b0243:	4c 8b 7f 10                                     	mov    0x10(%rdi),%r15
  1b0247:	4c 3b 3f                                        	cmp    (%rdi),%r15
  1b024a:	75 18                                           	jne    1b0264 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x944>
  1b024c:	48 8b 3c 24                                     	mov    (%rsp),%rdi
  1b0250:	44 89 44 24 14                                  	mov    %r8d,0x14(%rsp)
  1b0255:	ff 15 95 db 0b 00                               	call   *0xbdb95(%rip)        # 26ddf0 <_DYNAMIC+0x270>
  1b025b:	44 8b 44 24 14                                  	mov    0x14(%rsp),%r8d
  1b0260:	48 8b 3c 24                                     	mov    (%rsp),%rdi
  1b0264:	48 8b 47 08                                     	mov    0x8(%rdi),%rax
  1b0268:	42 c6 04 38 ff                                  	movb   $0xff,(%rax,%r15,1)
  1b026d:	49 ff c7                                        	inc    %r15
  1b0270:	4c 89 7f 10                                     	mov    %r15,0x10(%rdi)
  1b0274:	41 c1 e8 14                                     	shr    $0x14,%r8d
  1b0278:	b8 07 00 00 00                                  	mov    $0x7,%eax
  1b027d:	b9 fe ff 0f 00                                  	mov    $0xffffe,%ecx
  1b0282:	e9 06 ff ff ff                                  	jmp    1b018d <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x86d>
  1b0287:	89 d1                                           	mov    %edx,%ecx
  1b0289:	0f b6 7b 40                                     	movzbl 0x40(%rbx),%edi
  1b028d:	48 83 ff 2e                                     	cmp    $0x2e,%rdi
  1b0291:	0f 87 33 09 00 00                               	ja     1b0bca <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x12aa>
  1b0297:	4c 8b 5c 24 20                                  	mov    0x20(%rsp),%r11
  1b029c:	45 89 d9                                        	mov    %r11d,%r9d
  1b029f:	41 d1 e9                                        	shr    $1,%r9d
  1b02a2:	0f b6 73 41                                     	movzbl 0x41(%rbx),%esi
  1b02a6:	4c 8d 15 c3 22 e7 ff                            	lea    -0x18dd3d(%rip),%r10        # 22570 <anon.c3db339937c4b26e029c5c277d8a0513.101.llvm.11782617808337995929+0x31>
  1b02ad:	41 8b 04 fa                                     	mov    (%r10,%rdi,8),%eax
  1b02b1:	41 0f b6 54 fa 04                               	movzbl 0x4(%r10,%rdi,8),%edx
  1b02b7:	45 0f b6 44 fa 05                               	movzbl 0x5(%r10,%rdi,8),%r8d
  1b02bd:	41 0f b6 7c fa 06                               	movzbl 0x6(%r10,%rdi,8),%edi
  1b02c3:	29 c1                                           	sub    %eax,%ecx
  1b02c5:	89 4b 10                                        	mov    %ecx,0x10(%rbx)
  1b02c8:	41 39 f1                                        	cmp    %esi,%r9d
  1b02cb:	75 0e                                           	jne    1b02db <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x9bb>
  1b02cd:	66 85 c9                                        	test   %cx,%cx
  1b02d0:	78 14                                           	js     1b02e6 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x9c6>
  1b02d2:	39 c1                                           	cmp    %eax,%ecx
  1b02d4:	73 31                                           	jae    1b0307 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x9e7>
  1b02d6:	89 43 10                                        	mov    %eax,0x10(%rbx)
  1b02d9:	eb 31                                           	jmp    1b030c <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x9ec>
  1b02db:	39 c1                                           	cmp    %eax,%ecx
  1b02dd:	73 0f                                           	jae    1b02ee <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x9ce>
  1b02df:	01 43 14                                        	add    %eax,0x14(%rbx)
  1b02e2:	89 c8                                           	mov    %ecx,%eax
  1b02e4:	eb 0b                                           	jmp    1b02f1 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x9d1>
  1b02e6:	01 43 14                                        	add    %eax,0x14(%rbx)
  1b02e9:	e9 90 01 00 00                                  	jmp    1b047e <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xb5e>
  1b02ee:	89 43 10                                        	mov    %eax,0x10(%rbx)
  1b02f1:	44 88 43 40                                     	mov    %r8b,0x40(%rbx)
  1b02f5:	40 84 ff                                        	test   %dil,%dil
  1b02f8:	74 08                                           	je     1b0302 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x9e2>
  1b02fa:	40 80 f6 01                                     	xor    $0x1,%sil
  1b02fe:	40 88 73 41                                     	mov    %sil,0x41(%rbx)
  1b0302:	44 89 c2                                        	mov    %r8d,%edx
  1b0305:	eb 08                                           	jmp    1b030f <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x9ef>
  1b0307:	01 43 14                                        	add    %eax,0x14(%rbx)
  1b030a:	89 c8                                           	mov    %ecx,%eax
  1b030c:	88 53 40                                        	mov    %dl,0x40(%rbx)
  1b030f:	66 85 c0                                        	test   %ax,%ax
  1b0312:	0f 88 29 01 00 00                               	js     1b0441 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xb21>
  1b0318:	48 8b 73 08                                     	mov    0x8(%rbx),%rsi
  1b031c:	0f b6 53 45                                     	movzbl 0x45(%rbx),%edx
  1b0320:	0f b6 7b 46                                     	movzbl 0x46(%rbx),%edi
  1b0324:	44 8b 73 14                                     	mov    0x14(%rbx),%r14d
  1b0328:	8b 4b 18                                        	mov    0x18(%rbx),%ecx
  1b032b:	48 89 34 24                                     	mov    %rsi,(%rsp)
  1b032f:	eb 3f                                           	jmp    1b0370 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xa50>
  1b0331:	48 8b 46 08                                     	mov    0x8(%rsi),%rax
  1b0335:	42 88 3c 38                                     	mov    %dil,(%rax,%r15,1)
  1b0339:	49 ff c7                                        	inc    %r15
  1b033c:	4c 89 7e 10                                     	mov    %r15,0x10(%rsi)
  1b0340:	44 89 f7                                        	mov    %r14d,%edi
  1b0343:	c1 ef 13                                        	shr    $0x13,%edi
  1b0346:	b9 08 00 00 00                                  	mov    $0x8,%ecx
  1b034b:	b8 fe ff 07 00                                  	mov    $0x7fffe,%eax
  1b0350:	c6 43 45 01                                     	movb   $0x1,0x45(%rbx)
  1b0354:	40 88 7b 46                                     	mov    %dil,0x46(%rbx)
  1b0358:	41 21 c6                                        	and    %eax,%r14d
  1b035b:	44 89 73 14                                     	mov    %r14d,0x14(%rbx)
  1b035f:	89 4b 18                                        	mov    %ecx,0x18(%rbx)
  1b0362:	8b 43 10                                        	mov    0x10(%rbx),%eax
  1b0365:	b2 01                                           	mov    $0x1,%dl
  1b0367:	66 85 c0                                        	test   %ax,%ax
  1b036a:	0f 88 d5 00 00 00                               	js     1b0445 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xb25>
  1b0370:	01 c0                                           	add    %eax,%eax
  1b0372:	89 43 10                                        	mov    %eax,0x10(%rbx)
  1b0375:	45 01 f6                                        	add    %r14d,%r14d
  1b0378:	44 89 73 14                                     	mov    %r14d,0x14(%rbx)
  1b037c:	ff c9                                           	dec    %ecx
  1b037e:	89 4b 18                                        	mov    %ecx,0x18(%rbx)
  1b0381:	75 e4                                           	jne    1b0367 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xa47>
  1b0383:	c6 43 45 00                                     	movb   $0x0,0x45(%rbx)
  1b0387:	80 fa 01                                        	cmp    $0x1,%dl
  1b038a:	75 b4                                           	jne    1b0340 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xa20>
  1b038c:	40 80 ff ff                                     	cmp    $0xff,%dil
  1b0390:	74 36                                           	je     1b03c8 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xaa8>
  1b0392:	41 81 fe 00 00 00 08                            	cmp    $0x8000000,%r14d
  1b0399:	72 09                                           	jb     1b03a4 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xa84>
  1b039b:	40 fe c7                                        	inc    %dil
  1b039e:	40 80 ff ff                                     	cmp    $0xff,%dil
  1b03a2:	74 50                                           	je     1b03f4 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xad4>
  1b03a4:	4c 8b 7e 10                                     	mov    0x10(%rsi),%r15
  1b03a8:	4c 3b 3e                                        	cmp    (%rsi),%r15
  1b03ab:	75 84                                           	jne    1b0331 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xa11>
  1b03ad:	89 7c 24 14                                     	mov    %edi,0x14(%rsp)
  1b03b1:	48 8b 3c 24                                     	mov    (%rsp),%rdi
  1b03b5:	ff 15 35 da 0b 00                               	call   *0xbda35(%rip)        # 26ddf0 <_DYNAMIC+0x270>
  1b03bb:	8b 7c 24 14                                     	mov    0x14(%rsp),%edi
  1b03bf:	48 8b 34 24                                     	mov    (%rsp),%rsi
  1b03c3:	e9 69 ff ff ff                                  	jmp    1b0331 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xa11>
  1b03c8:	4c 8b 7e 10                                     	mov    0x10(%rsi),%r15
  1b03cc:	4c 3b 3e                                        	cmp    (%rsi),%r15
  1b03cf:	75 0e                                           	jne    1b03df <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xabf>
  1b03d1:	48 8b 3c 24                                     	mov    (%rsp),%rdi
  1b03d5:	ff 15 15 da 0b 00                               	call   *0xbda15(%rip)        # 26ddf0 <_DYNAMIC+0x270>
  1b03db:	48 8b 34 24                                     	mov    (%rsp),%rsi
  1b03df:	48 8b 46 08                                     	mov    0x8(%rsi),%rax
  1b03e3:	42 c6 04 38 ff                                  	movb   $0xff,(%rax,%r15,1)
  1b03e8:	49 ff c7                                        	inc    %r15
  1b03eb:	4c 89 7e 10                                     	mov    %r15,0x10(%rsi)
  1b03ef:	44 89 f7                                        	mov    %r14d,%edi
  1b03f2:	eb 3b                                           	jmp    1b042f <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xb0f>
  1b03f4:	44 89 f7                                        	mov    %r14d,%edi
  1b03f7:	81 e7 fe ff ff 07                               	and    $0x7fffffe,%edi
  1b03fd:	89 7b 14                                        	mov    %edi,0x14(%rbx)
  1b0400:	4c 8b 7e 10                                     	mov    0x10(%rsi),%r15
  1b0404:	4c 3b 3e                                        	cmp    (%rsi),%r15
  1b0407:	75 16                                           	jne    1b041f <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xaff>
  1b0409:	89 7c 24 14                                     	mov    %edi,0x14(%rsp)
  1b040d:	48 8b 3c 24                                     	mov    (%rsp),%rdi
  1b0411:	ff 15 d9 d9 0b 00                               	call   *0xbd9d9(%rip)        # 26ddf0 <_DYNAMIC+0x270>
  1b0417:	8b 7c 24 14                                     	mov    0x14(%rsp),%edi
  1b041b:	48 8b 34 24                                     	mov    (%rsp),%rsi
  1b041f:	48 8b 46 08                                     	mov    0x8(%rsi),%rax
  1b0423:	42 c6 04 38 ff                                  	movb   $0xff,(%rax,%r15,1)
  1b0428:	49 ff c7                                        	inc    %r15
  1b042b:	4c 89 7e 10                                     	mov    %r15,0x10(%rsi)
  1b042f:	c1 ef 14                                        	shr    $0x14,%edi
  1b0432:	b9 07 00 00 00                                  	mov    $0x7,%ecx
  1b0437:	b8 fe ff 0f 00                                  	mov    $0xffffe,%eax
  1b043c:	e9 0f ff ff ff                                  	jmp    1b0350 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xa30>
  1b0441:	89 c1                                           	mov    %eax,%ecx
  1b0443:	eb 0b                                           	jmp    1b0450 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xb30>
  1b0445:	0f b6 53 40                                     	movzbl 0x40(%rbx),%edx
  1b0449:	89 c1                                           	mov    %eax,%ecx
  1b044b:	4c 8b 5c 24 20                                  	mov    0x20(%rsp),%r11
  1b0450:	0f b6 fa                                        	movzbl %dl,%edi
  1b0453:	40 80 ff 2e                                     	cmp    $0x2e,%dil
  1b0457:	0f 87 6d 07 00 00                               	ja     1b0bca <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x12aa>
  1b045d:	0f b6 73 41                                     	movzbl 0x41(%rbx),%esi
  1b0461:	4c 8d 0d 08 21 e7 ff                            	lea    -0x18def8(%rip),%r9        # 22570 <anon.c3db339937c4b26e029c5c277d8a0513.101.llvm.11782617808337995929+0x31>
  1b0468:	41 8b 04 f9                                     	mov    (%r9,%rdi,8),%eax
  1b046c:	41 0f b6 54 f9 04                               	movzbl 0x4(%r9,%rdi,8),%edx
  1b0472:	45 0f b6 44 f9 05                               	movzbl 0x5(%r9,%rdi,8),%r8d
  1b0478:	41 0f b6 7c f9 06                               	movzbl 0x6(%r9,%rdi,8),%edi
  1b047e:	45 89 d9                                        	mov    %r11d,%r9d
  1b0481:	41 83 e1 01                                     	and    $0x1,%r9d
  1b0485:	29 c1                                           	sub    %eax,%ecx
  1b0487:	89 4b 10                                        	mov    %ecx,0x10(%rbx)
  1b048a:	44 0f b6 d6                                     	movzbl %sil,%r10d
  1b048e:	45 39 d1                                        	cmp    %r10d,%r9d
  1b0491:	75 0e                                           	jne    1b04a1 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xb81>
  1b0493:	66 85 c9                                        	test   %cx,%cx
  1b0496:	78 14                                           	js     1b04ac <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xb8c>
  1b0498:	39 c1                                           	cmp    %eax,%ecx
  1b049a:	73 2e                                           	jae    1b04ca <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xbaa>
  1b049c:	89 43 10                                        	mov    %eax,0x10(%rbx)
  1b049f:	eb 2e                                           	jmp    1b04cf <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xbaf>
  1b04a1:	39 c1                                           	cmp    %eax,%ecx
  1b04a3:	73 0f                                           	jae    1b04b4 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xb94>
  1b04a5:	01 43 14                                        	add    %eax,0x14(%rbx)
  1b04a8:	89 c8                                           	mov    %ecx,%eax
  1b04aa:	eb 0b                                           	jmp    1b04b7 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xb97>
  1b04ac:	01 43 14                                        	add    %eax,0x14(%rbx)
  1b04af:	e9 45 01 00 00                                  	jmp    1b05f9 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xcd9>
  1b04b4:	89 43 10                                        	mov    %eax,0x10(%rbx)
  1b04b7:	44 88 43 40                                     	mov    %r8b,0x40(%rbx)
  1b04bb:	40 84 ff                                        	test   %dil,%dil
  1b04be:	74 12                                           	je     1b04d2 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xbb2>
  1b04c0:	40 80 f6 01                                     	xor    $0x1,%sil
  1b04c4:	40 88 73 41                                     	mov    %sil,0x41(%rbx)
  1b04c8:	eb 08                                           	jmp    1b04d2 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xbb2>
  1b04ca:	01 43 14                                        	add    %eax,0x14(%rbx)
  1b04cd:	89 c8                                           	mov    %ecx,%eax
  1b04cf:	88 53 40                                        	mov    %dl,0x40(%rbx)
  1b04d2:	66 85 c0                                        	test   %ax,%ax
  1b04d5:	0f 88 1e 01 00 00                               	js     1b05f9 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xcd9>
  1b04db:	48 8b 73 08                                     	mov    0x8(%rbx),%rsi
  1b04df:	0f b6 53 45                                     	movzbl 0x45(%rbx),%edx
  1b04e3:	44 0f b6 73 46                                  	movzbl 0x46(%rbx),%r14d
  1b04e8:	44 8b 6b 14                                     	mov    0x14(%rbx),%r13d
  1b04ec:	8b 4b 18                                        	mov    0x18(%rbx),%ecx
  1b04ef:	48 89 34 24                                     	mov    %rsi,(%rsp)
  1b04f3:	eb 40                                           	jmp    1b0535 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xc15>
  1b04f5:	48 8b 46 08                                     	mov    0x8(%rsi),%rax
  1b04f9:	46 88 34 38                                     	mov    %r14b,(%rax,%r15,1)
  1b04fd:	49 ff c7                                        	inc    %r15
  1b0500:	4c 89 7e 10                                     	mov    %r15,0x10(%rsi)
  1b0504:	45 89 ee                                        	mov    %r13d,%r14d
  1b0507:	41 c1 ee 13                                     	shr    $0x13,%r14d
  1b050b:	b9 08 00 00 00                                  	mov    $0x8,%ecx
  1b0510:	b8 fe ff 07 00                                  	mov    $0x7fffe,%eax
  1b0515:	c6 43 45 01                                     	movb   $0x1,0x45(%rbx)
  1b0519:	44 88 73 46                                     	mov    %r14b,0x46(%rbx)
  1b051d:	41 21 c5                                        	and    %eax,%r13d
  1b0520:	44 89 6b 14                                     	mov    %r13d,0x14(%rbx)
  1b0524:	89 4b 18                                        	mov    %ecx,0x18(%rbx)
  1b0527:	8b 43 10                                        	mov    0x10(%rbx),%eax
  1b052a:	b2 01                                           	mov    $0x1,%dl
  1b052c:	66 85 c0                                        	test   %ax,%ax
  1b052f:	0f 88 c4 00 00 00                               	js     1b05f9 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xcd9>
  1b0535:	01 c0                                           	add    %eax,%eax
  1b0537:	89 43 10                                        	mov    %eax,0x10(%rbx)
  1b053a:	45 01 ed                                        	add    %r13d,%r13d
  1b053d:	44 89 6b 14                                     	mov    %r13d,0x14(%rbx)
  1b0541:	ff c9                                           	dec    %ecx
  1b0543:	89 4b 18                                        	mov    %ecx,0x18(%rbx)
  1b0546:	75 e4                                           	jne    1b052c <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xc0c>
  1b0548:	c6 43 45 00                                     	movb   $0x0,0x45(%rbx)
  1b054c:	80 fa 01                                        	cmp    $0x1,%dl
  1b054f:	75 b3                                           	jne    1b0504 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xbe4>
  1b0551:	41 80 fe ff                                     	cmp    $0xff,%r14b
  1b0555:	74 2e                                           	je     1b0585 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xc65>
  1b0557:	41 81 fd 00 00 00 08                            	cmp    $0x8000000,%r13d
  1b055e:	72 09                                           	jb     1b0569 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xc49>
  1b0560:	41 fe c6                                        	inc    %r14b
  1b0563:	41 80 fe ff                                     	cmp    $0xff,%r14b
  1b0567:	74 48                                           	je     1b05b1 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xc91>
  1b0569:	4c 8b 7e 10                                     	mov    0x10(%rsi),%r15
  1b056d:	4c 3b 3e                                        	cmp    (%rsi),%r15
  1b0570:	75 83                                           	jne    1b04f5 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xbd5>
  1b0572:	48 8b 3c 24                                     	mov    (%rsp),%rdi
  1b0576:	ff 15 74 d8 0b 00                               	call   *0xbd874(%rip)        # 26ddf0 <_DYNAMIC+0x270>
  1b057c:	48 8b 34 24                                     	mov    (%rsp),%rsi
  1b0580:	e9 70 ff ff ff                                  	jmp    1b04f5 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xbd5>
  1b0585:	4c 8b 76 10                                     	mov    0x10(%rsi),%r14
  1b0589:	4c 3b 36                                        	cmp    (%rsi),%r14
  1b058c:	75 0e                                           	jne    1b059c <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xc7c>
  1b058e:	48 8b 3c 24                                     	mov    (%rsp),%rdi
  1b0592:	ff 15 58 d8 0b 00                               	call   *0xbd858(%rip)        # 26ddf0 <_DYNAMIC+0x270>
  1b0598:	48 8b 34 24                                     	mov    (%rsp),%rsi
  1b059c:	48 8b 46 08                                     	mov    0x8(%rsi),%rax
  1b05a0:	42 c6 04 30 ff                                  	movb   $0xff,(%rax,%r14,1)
  1b05a5:	49 ff c6                                        	inc    %r14
  1b05a8:	4c 89 76 10                                     	mov    %r14,0x10(%rsi)
  1b05ac:	45 89 ee                                        	mov    %r13d,%r14d
  1b05af:	eb 35                                           	jmp    1b05e6 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xcc6>
  1b05b1:	45 89 ee                                        	mov    %r13d,%r14d
  1b05b4:	41 81 e6 fe ff ff 07                            	and    $0x7fffffe,%r14d
  1b05bb:	44 89 73 14                                     	mov    %r14d,0x14(%rbx)
  1b05bf:	4c 8b 7e 10                                     	mov    0x10(%rsi),%r15
  1b05c3:	4c 3b 3e                                        	cmp    (%rsi),%r15
  1b05c6:	75 0e                                           	jne    1b05d6 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xcb6>
  1b05c8:	48 8b 3c 24                                     	mov    (%rsp),%rdi
  1b05cc:	ff 15 1e d8 0b 00                               	call   *0xbd81e(%rip)        # 26ddf0 <_DYNAMIC+0x270>
  1b05d2:	48 8b 34 24                                     	mov    (%rsp),%rsi
  1b05d6:	48 8b 46 08                                     	mov    0x8(%rsi),%rax
  1b05da:	42 c6 04 38 ff                                  	movb   $0xff,(%rax,%r15,1)
  1b05df:	49 ff c7                                        	inc    %r15
  1b05e2:	4c 89 7e 10                                     	mov    %r15,0x10(%rsi)
  1b05e6:	41 c1 ee 14                                     	shr    $0x14,%r14d
  1b05ea:	b9 07 00 00 00                                  	mov    $0x7,%ecx
  1b05ef:	b8 fe ff 0f 00                                  	mov    $0xffffe,%eax
  1b05f4:	e9 1c ff ff ff                                  	jmp    1b0515 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xbf5>
  1b05f9:	4c 8b 74 24 20                                  	mov    0x20(%rsp),%r14
  1b05fe:	4d 89 f5                                        	mov    %r14,%r13
  1b0601:	4c 0f af 6c 24 18                               	imul   0x18(%rsp),%r13
  1b0607:	49 01 ed                                        	add    %rbp,%r13
  1b060a:	4c 89 ef                                        	mov    %r13,%rdi
  1b060d:	48 8b 74 24 50                                  	mov    0x50(%rsp),%rsi
  1b0612:	48 89 da                                        	mov    %rbx,%rdx
  1b0615:	e8 06 f0 ff ff                                  	call   1af620 <emuella_j2k_tier1::encode_sign_bit_at::<false>>
  1b061a:	4d 39 e5                                        	cmp    %r12,%r13
  1b061d:	0f 83 01 06 00 00                               	jae    1b0c24 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x1304>
  1b0623:	4a 8d 04 6d 00 00 00 00                         	lea    0x0(,%r13,2),%rax
  1b062b:	4c 01 e8                                        	add    %r13,%rax
  1b062e:	48 8b 7c 24 08                                  	mov    0x8(%rsp),%rdi
  1b0633:	c6 04 07 01                                     	movb   $0x1,(%rdi,%rax,1)
  1b0637:	80 7c 24 38 00                                  	cmpb   $0x0,0x38(%rsp)
  1b063c:	75 1f                                           	jne    1b065d <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xd3d>
  1b063e:	e9 4d f4 ff ff                                  	jmp    1afa90 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x170>
  1b0643:	66 66 66 66 2e 0f 1f 84 00 00 00 00 00          	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  1b0650:	49 ff c6                                        	inc    %r14
  1b0653:	49 83 fe 03                                     	cmp    $0x3,%r14
  1b0657:	0f 84 33 f4 ff ff                               	je     1afa90 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x170>
  1b065d:	4c 03 6c 24 18                                  	add    0x18(%rsp),%r13
  1b0662:	4d 39 e5                                        	cmp    %r12,%r13
  1b0665:	0f 83 24 05 00 00                               	jae    1b0b8f <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x126f>
  1b066b:	4e 8d 3c 6d 00 00 00 00                         	lea    0x0(,%r13,2),%r15
  1b0673:	4d 01 ef                                        	add    %r13,%r15
  1b0676:	42 80 3c 3f 00                                  	cmpb   $0x0,(%rdi,%r15,1)
  1b067b:	75 d3                                           	jne    1b0650 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xd30>
  1b067d:	49 01 ff                                        	add    %rdi,%r15
  1b0680:	41 80 7f 01 00                                  	cmpb   $0x0,0x1(%r15)
  1b0685:	75 c9                                           	jne    1b0650 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xd30>
  1b0687:	4c 89 74 24 20                                  	mov    %r14,0x20(%rsp)
  1b068c:	4c 3b 6c 24 28                                  	cmp    0x28(%rsp),%r13
  1b0691:	0f 83 1e 05 00 00                               	jae    1b0bb5 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x1295>
  1b0697:	48 8b 44 24 40                                  	mov    0x40(%rsp),%rax
  1b069c:	42 8b 04 a8                                     	mov    (%rax,%r13,4),%eax
  1b06a0:	31 ed                                           	xor    %ebp,%ebp
  1b06a2:	8b 4c 24 5c                                     	mov    0x5c(%rsp),%ecx
  1b06a6:	0f a3 c8                                        	bt     %ecx,%eax
  1b06a9:	41 0f 92 c6                                     	setb   %r14b
  1b06ad:	4c 89 e6                                        	mov    %r12,%rsi
  1b06b0:	4c 89 ea                                        	mov    %r13,%rdx
  1b06b3:	48 8b 4c 24 18                                  	mov    0x18(%rsp),%rcx
  1b06b8:	e8 13 ec ff ff                                  	call   1af2d0 <emuella_j2k_tier1::neighborhood_at::<false>>
  1b06bd:	89 c1                                           	mov    %eax,%ecx
  1b06bf:	c1 e9 18                                        	shr    $0x18,%ecx
  1b06c2:	83 e1 01                                        	and    $0x1,%ecx
  1b06c5:	89 c2                                           	mov    %eax,%edx
  1b06c7:	c1 ea 08                                        	shr    $0x8,%edx
  1b06ca:	83 e2 01                                        	and    $0x1,%edx
  1b06cd:	48 0f ba e0 30                                  	bt     $0x30,%rax
  1b06d2:	48 83 d2 00                                     	adc    $0x0,%rdx
  1b06d6:	89 c6                                           	mov    %eax,%esi
  1b06d8:	83 e6 01                                        	and    $0x1,%esi
  1b06db:	89 c7                                           	mov    %eax,%edi
  1b06dd:	c1 ef 10                                        	shr    $0x10,%edi
  1b06e0:	83 e7 01                                        	and    $0x1,%edi
  1b06e3:	49 89 c0                                        	mov    %rax,%r8
  1b06e6:	49 c1 e8 38                                     	shr    $0x38,%r8
  1b06ea:	49 01 f0                                        	add    %rsi,%r8
  1b06ed:	48 0f ba e0 28                                  	bt     $0x28,%rax
  1b06f2:	49 11 f8                                        	adc    %rdi,%r8
  1b06f5:	48 8d 14 52                                     	lea    (%rdx,%rdx,2),%rdx
  1b06f9:	48 0f ba e0 20                                  	bt     $0x20,%rax
  1b06fe:	48 13 4c 24 70                                  	adc    0x70(%rsp),%rcx
  1b0703:	4b 8d 04 c0                                     	lea    (%r8,%r8,8),%rax
  1b0707:	48 01 d1                                        	add    %rdx,%rcx
  1b070a:	0f b6 3c 01                                     	movzbl (%rcx,%rax,1),%edi
  1b070e:	48 83 ff 12                                     	cmp    $0x12,%rdi
  1b0712:	0f 87 02 04 00 00                               	ja     1b0b1a <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x11fa>
  1b0718:	0f b6 44 7b 1c                                  	movzbl 0x1c(%rbx,%rdi,2),%eax
  1b071d:	48 83 f8 2e                                     	cmp    $0x2e,%rax
  1b0721:	0f 87 05 04 00 00                               	ja     1b0b2c <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x120c>
  1b0727:	44 88 f5                                        	mov    %r14b,%bpl
  1b072a:	0f b6 54 7b 1d                                  	movzbl 0x1d(%rbx,%rdi,2),%edx
  1b072f:	4c 8d 0d 3a 1e e7 ff                            	lea    -0x18e1c6(%rip),%r9        # 22570 <anon.c3db339937c4b26e029c5c277d8a0513.101.llvm.11782617808337995929+0x31>
  1b0736:	41 8b 0c c1                                     	mov    (%r9,%rax,8),%ecx
  1b073a:	41 0f b6 74 c1 04                               	movzbl 0x4(%r9,%rax,8),%esi
  1b0740:	45 0f b6 44 c1 05                               	movzbl 0x5(%r9,%rax,8),%r8d
  1b0746:	45 0f b6 4c c1 06                               	movzbl 0x6(%r9,%rax,8),%r9d
  1b074c:	8b 43 10                                        	mov    0x10(%rbx),%eax
  1b074f:	29 c8                                           	sub    %ecx,%eax
  1b0751:	89 43 10                                        	mov    %eax,0x10(%rbx)
  1b0754:	39 d5                                           	cmp    %edx,%ebp
  1b0756:	4c 89 2c 24                                     	mov    %r13,(%rsp)
  1b075a:	75 13                                           	jne    1b076f <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xe4f>
  1b075c:	66 85 c0                                        	test   %ax,%ax
  1b075f:	78 19                                           	js     1b077a <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xe5a>
  1b0761:	39 c8                                           	cmp    %ecx,%eax
  1b0763:	48 8b 54 24 08                                  	mov    0x8(%rsp),%rdx
  1b0768:	73 41                                           	jae    1b07ab <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xe8b>
  1b076a:	89 4b 10                                        	mov    %ecx,0x10(%rbx)
  1b076d:	eb 41                                           	jmp    1b07b0 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xe90>
  1b076f:	39 c8                                           	cmp    %ecx,%eax
  1b0771:	73 14                                           	jae    1b0787 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xe67>
  1b0773:	01 4b 14                                        	add    %ecx,0x14(%rbx)
  1b0776:	89 c1                                           	mov    %eax,%ecx
  1b0778:	eb 10                                           	jmp    1b078a <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xe6a>
  1b077a:	01 4b 14                                        	add    %ecx,0x14(%rbx)
  1b077d:	48 8b 7c 24 08                                  	mov    0x8(%rsp),%rdi
  1b0782:	e9 77 01 00 00                                  	jmp    1b08fe <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xfde>
  1b0787:	89 4b 10                                        	mov    %ecx,0x10(%rbx)
  1b078a:	44 88 44 7b 1c                                  	mov    %r8b,0x1c(%rbx,%rdi,2)
  1b078f:	45 84 c9                                        	test   %r9b,%r9b
  1b0792:	74 07                                           	je     1b079b <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xe7b>
  1b0794:	80 f2 01                                        	xor    $0x1,%dl
  1b0797:	88 54 7b 1d                                     	mov    %dl,0x1d(%rbx,%rdi,2)
  1b079b:	48 8b 7c 24 08                                  	mov    0x8(%rsp),%rdi
  1b07a0:	66 85 c9                                        	test   %cx,%cx
  1b07a3:	0f 88 55 01 00 00                               	js     1b08fe <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xfde>
  1b07a9:	eb 16                                           	jmp    1b07c1 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xea1>
  1b07ab:	01 4b 14                                        	add    %ecx,0x14(%rbx)
  1b07ae:	89 c1                                           	mov    %eax,%ecx
  1b07b0:	40 88 74 7b 1c                                  	mov    %sil,0x1c(%rbx,%rdi,2)
  1b07b5:	48 89 d7                                        	mov    %rdx,%rdi
  1b07b8:	66 85 c9                                        	test   %cx,%cx
  1b07bb:	0f 88 3d 01 00 00                               	js     1b08fe <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xfde>
  1b07c1:	4c 8b 6b 08                                     	mov    0x8(%rbx),%r13
  1b07c5:	0f b6 53 45                                     	movzbl 0x45(%rbx),%edx
  1b07c9:	44 0f b6 63 46                                  	movzbl 0x46(%rbx),%r12d
  1b07ce:	44 8b 73 14                                     	mov    0x14(%rbx),%r14d
  1b07d2:	8b 43 18                                        	mov    0x18(%rbx),%eax
  1b07d5:	89 6c 24 38                                     	mov    %ebp,0x38(%rsp)
  1b07d9:	eb 44                                           	jmp    1b081f <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xeff>
  1b07db:	49 8b 45 08                                     	mov    0x8(%r13),%rax
  1b07df:	44 88 24 28                                     	mov    %r12b,(%rax,%rbp,1)
  1b07e3:	48 ff c5                                        	inc    %rbp
  1b07e6:	49 89 6d 10                                     	mov    %rbp,0x10(%r13)
  1b07ea:	8b 6c 24 38                                     	mov    0x38(%rsp),%ebp
  1b07ee:	45 89 f4                                        	mov    %r14d,%r12d
  1b07f1:	41 c1 ec 13                                     	shr    $0x13,%r12d
  1b07f5:	b8 08 00 00 00                                  	mov    $0x8,%eax
  1b07fa:	b9 fe ff 07 00                                  	mov    $0x7fffe,%ecx
  1b07ff:	c6 43 45 01                                     	movb   $0x1,0x45(%rbx)
  1b0803:	44 88 63 46                                     	mov    %r12b,0x46(%rbx)
  1b0807:	41 21 ce                                        	and    %ecx,%r14d
  1b080a:	44 89 73 14                                     	mov    %r14d,0x14(%rbx)
  1b080e:	89 43 18                                        	mov    %eax,0x18(%rbx)
  1b0811:	8b 4b 10                                        	mov    0x10(%rbx),%ecx
  1b0814:	b2 01                                           	mov    $0x1,%dl
  1b0816:	66 85 c9                                        	test   %cx,%cx
  1b0819:	0f 88 df 00 00 00                               	js     1b08fe <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xfde>
  1b081f:	01 c9                                           	add    %ecx,%ecx
  1b0821:	89 4b 10                                        	mov    %ecx,0x10(%rbx)
  1b0824:	45 01 f6                                        	add    %r14d,%r14d
  1b0827:	44 89 73 14                                     	mov    %r14d,0x14(%rbx)
  1b082b:	ff c8                                           	dec    %eax
  1b082d:	89 43 18                                        	mov    %eax,0x18(%rbx)
  1b0830:	75 e4                                           	jne    1b0816 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xef6>
  1b0832:	c6 43 45 00                                     	movb   $0x0,0x45(%rbx)
  1b0836:	80 fa 01                                        	cmp    $0x1,%dl
  1b0839:	75 b3                                           	jne    1b07ee <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xece>
  1b083b:	41 80 fc ff                                     	cmp    $0xff,%r12b
  1b083f:	74 33                                           	je     1b0874 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xf54>
  1b0841:	41 81 fe 00 00 00 08                            	cmp    $0x8000000,%r14d
  1b0848:	72 09                                           	jb     1b0853 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xf33>
  1b084a:	41 fe c4                                        	inc    %r12b
  1b084d:	41 80 fc ff                                     	cmp    $0xff,%r12b
  1b0851:	74 5f                                           	je     1b08b2 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xf92>
  1b0853:	49 8b 6d 10                                     	mov    0x10(%r13),%rbp
  1b0857:	49 3b 6d 00                                     	cmp    0x0(%r13),%rbp
  1b085b:	0f 85 7a ff ff ff                               	jne    1b07db <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xebb>
  1b0861:	4c 89 ef                                        	mov    %r13,%rdi
  1b0864:	ff 15 86 d5 0b 00                               	call   *0xbd586(%rip)        # 26ddf0 <_DYNAMIC+0x270>
  1b086a:	48 8b 7c 24 08                                  	mov    0x8(%rsp),%rdi
  1b086f:	e9 67 ff ff ff                                  	jmp    1b07db <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xebb>
  1b0874:	4d 8b 65 10                                     	mov    0x10(%r13),%r12
  1b0878:	4d 3b 65 00                                     	cmp    0x0(%r13),%r12
  1b087c:	75 0e                                           	jne    1b088c <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xf6c>
  1b087e:	4c 89 ef                                        	mov    %r13,%rdi
  1b0881:	ff 15 69 d5 0b 00                               	call   *0xbd569(%rip)        # 26ddf0 <_DYNAMIC+0x270>
  1b0887:	48 8b 7c 24 08                                  	mov    0x8(%rsp),%rdi
  1b088c:	49 8b 45 08                                     	mov    0x8(%r13),%rax
  1b0890:	42 c6 04 20 ff                                  	movb   $0xff,(%rax,%r12,1)
  1b0895:	49 ff c4                                        	inc    %r12
  1b0898:	4d 89 65 10                                     	mov    %r12,0x10(%r13)
  1b089c:	45 89 f4                                        	mov    %r14d,%r12d
  1b089f:	41 c1 ec 14                                     	shr    $0x14,%r12d
  1b08a3:	b8 07 00 00 00                                  	mov    $0x7,%eax
  1b08a8:	b9 fe ff 0f 00                                  	mov    $0xffffe,%ecx
  1b08ad:	e9 4d ff ff ff                                  	jmp    1b07ff <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xedf>
  1b08b2:	45 89 f4                                        	mov    %r14d,%r12d
  1b08b5:	41 81 e4 fe ff ff 07                            	and    $0x7fffffe,%r12d
  1b08bc:	44 89 63 14                                     	mov    %r12d,0x14(%rbx)
  1b08c0:	49 8b 6d 10                                     	mov    0x10(%r13),%rbp
  1b08c4:	49 3b 6d 00                                     	cmp    0x0(%r13),%rbp
  1b08c8:	75 0e                                           	jne    1b08d8 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xfb8>
  1b08ca:	4c 89 ef                                        	mov    %r13,%rdi
  1b08cd:	ff 15 1d d5 0b 00                               	call   *0xbd51d(%rip)        # 26ddf0 <_DYNAMIC+0x270>
  1b08d3:	48 8b 7c 24 08                                  	mov    0x8(%rsp),%rdi
  1b08d8:	49 8b 45 08                                     	mov    0x8(%r13),%rax
  1b08dc:	c6 04 28 ff                                     	movb   $0xff,(%rax,%rbp,1)
  1b08e0:	48 ff c5                                        	inc    %rbp
  1b08e3:	49 89 6d 10                                     	mov    %rbp,0x10(%r13)
  1b08e7:	41 c1 ec 14                                     	shr    $0x14,%r12d
  1b08eb:	b8 07 00 00 00                                  	mov    $0x7,%eax
  1b08f0:	b9 fe ff 0f 00                                  	mov    $0xffffe,%ecx
  1b08f5:	8b 6c 24 38                                     	mov    0x38(%rsp),%ebp
  1b08f9:	e9 01 ff ff ff                                  	jmp    1b07ff <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xedf>
  1b08fe:	85 ed                                           	test   %ebp,%ebp
  1b0900:	4c 8b 64 24 68                                  	mov    0x68(%rsp),%r12
  1b0905:	4c 8b 74 24 20                                  	mov    0x20(%rsp),%r14
  1b090a:	4c 8b 2c 24                                     	mov    (%rsp),%r13
  1b090e:	0f 84 3c fd ff ff                               	je     1b0650 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xd30>
  1b0914:	4c 89 ef                                        	mov    %r13,%rdi
  1b0917:	48 8b 74 24 50                                  	mov    0x50(%rsp),%rsi
  1b091c:	48 89 da                                        	mov    %rbx,%rdx
  1b091f:	e8 fc ec ff ff                                  	call   1af620 <emuella_j2k_tier1::encode_sign_bit_at::<false>>
  1b0924:	48 8b 7c 24 08                                  	mov    0x8(%rsp),%rdi
  1b0929:	41 c6 07 01                                     	movb   $0x1,(%r15)
  1b092d:	e9 1e fd ff ff                                  	jmp    1b0650 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0xd30>
  1b0932:	0f b6 7b 3e                                     	movzbl 0x3e(%rbx),%edi
  1b0936:	48 83 ff 2f                                     	cmp    $0x2f,%rdi
  1b093a:	0f 83 8a 02 00 00                               	jae    1b0bca <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x12aa>
  1b0940:	0f b6 4b 3f                                     	movzbl 0x3f(%rbx),%ecx
  1b0944:	48 8d 35 25 1c e7 ff                            	lea    -0x18e3db(%rip),%rsi        # 22570 <anon.c3db339937c4b26e029c5c277d8a0513.101.llvm.11782617808337995929+0x31>
  1b094b:	8b 04 fe                                        	mov    (%rsi,%rdi,8),%eax
  1b094e:	0f b6 54 fe 04                                  	movzbl 0x4(%rsi,%rdi,8),%edx
  1b0953:	44 0f b6 44 fe 05                               	movzbl 0x5(%rsi,%rdi,8),%r8d
  1b0959:	0f b6 7c fe 06                                  	movzbl 0x6(%rsi,%rdi,8),%edi
  1b095e:	8b 73 10                                        	mov    0x10(%rbx),%esi
  1b0961:	29 c6                                           	sub    %eax,%esi
  1b0963:	89 73 10                                        	mov    %esi,0x10(%rbx)
  1b0966:	84 c9                                           	test   %cl,%cl
  1b0968:	74 0b                                           	je     1b0975 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x1055>
  1b096a:	39 c6                                           	cmp    %eax,%esi
  1b096c:	73 1e                                           	jae    1b098c <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x106c>
  1b096e:	01 43 14                                        	add    %eax,0x14(%rbx)
  1b0971:	89 f0                                           	mov    %esi,%eax
  1b0973:	eb 1a                                           	jmp    1b098f <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x106f>
  1b0975:	66 85 f6                                        	test   %si,%si
  1b0978:	0f 88 08 f1 ff ff                               	js     1afa86 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x166>
  1b097e:	39 c6                                           	cmp    %eax,%esi
  1b0980:	73 1e                                           	jae    1b09a0 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x1080>
  1b0982:	48 8b 4c 24 48                                  	mov    0x48(%rsp),%rcx
  1b0987:	89 41 10                                        	mov    %eax,0x10(%rcx)
  1b098a:	eb 1e                                           	jmp    1b09aa <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x108a>
  1b098c:	89 43 10                                        	mov    %eax,0x10(%rbx)
  1b098f:	44 88 43 3e                                     	mov    %r8b,0x3e(%rbx)
  1b0993:	40 84 ff                                        	test   %dil,%dil
  1b0996:	74 1f                                           	je     1b09b7 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x1097>
  1b0998:	80 f1 01                                        	xor    $0x1,%cl
  1b099b:	88 4b 3f                                        	mov    %cl,0x3f(%rbx)
  1b099e:	eb 17                                           	jmp    1b09b7 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x1097>
  1b09a0:	48 8b 4c 24 48                                  	mov    0x48(%rsp),%rcx
  1b09a5:	01 41 14                                        	add    %eax,0x14(%rcx)
  1b09a8:	89 f0                                           	mov    %esi,%eax
  1b09aa:	48 8b 5c 24 48                                  	mov    0x48(%rsp),%rbx
  1b09af:	88 53 3e                                        	mov    %dl,0x3e(%rbx)
  1b09b2:	4c 8b 64 24 68                                  	mov    0x68(%rsp),%r12
  1b09b7:	48 8b 7c 24 08                                  	mov    0x8(%rsp),%rdi
  1b09bc:	66 85 c0                                        	test   %ax,%ax
  1b09bf:	0f 88 cb f0 ff ff                               	js     1afa90 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x170>
  1b09c5:	4c 8b 6b 08                                     	mov    0x8(%rbx),%r13
  1b09c9:	0f b6 53 45                                     	movzbl 0x45(%rbx),%edx
  1b09cd:	44 0f b6 73 46                                  	movzbl 0x46(%rbx),%r14d
  1b09d2:	8b 6b 14                                        	mov    0x14(%rbx),%ebp
  1b09d5:	8b 4b 18                                        	mov    0x18(%rbx),%ecx
  1b09d8:	eb 3e                                           	jmp    1b0a18 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x10f8>
  1b09da:	49 8b 45 08                                     	mov    0x8(%r13),%rax
  1b09de:	46 88 34 38                                     	mov    %r14b,(%rax,%r15,1)
  1b09e2:	49 ff c7                                        	inc    %r15
  1b09e5:	4d 89 7d 10                                     	mov    %r15,0x10(%r13)
  1b09e9:	41 89 ee                                        	mov    %ebp,%r14d
  1b09ec:	41 c1 ee 13                                     	shr    $0x13,%r14d
  1b09f0:	b9 08 00 00 00                                  	mov    $0x8,%ecx
  1b09f5:	b8 fe ff 07 00                                  	mov    $0x7fffe,%eax
  1b09fa:	c6 43 45 01                                     	movb   $0x1,0x45(%rbx)
  1b09fe:	44 88 73 46                                     	mov    %r14b,0x46(%rbx)
  1b0a02:	21 c5                                           	and    %eax,%ebp
  1b0a04:	89 6b 14                                        	mov    %ebp,0x14(%rbx)
  1b0a07:	89 4b 18                                        	mov    %ecx,0x18(%rbx)
  1b0a0a:	8b 43 10                                        	mov    0x10(%rbx),%eax
  1b0a0d:	b2 01                                           	mov    $0x1,%dl
  1b0a0f:	66 85 c0                                        	test   %ax,%ax
  1b0a12:	0f 88 78 f0 ff ff                               	js     1afa90 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x170>
  1b0a18:	01 c0                                           	add    %eax,%eax
  1b0a1a:	89 43 10                                        	mov    %eax,0x10(%rbx)
  1b0a1d:	01 ed                                           	add    %ebp,%ebp
  1b0a1f:	89 6b 14                                        	mov    %ebp,0x14(%rbx)
  1b0a22:	ff c9                                           	dec    %ecx
  1b0a24:	89 4b 18                                        	mov    %ecx,0x18(%rbx)
  1b0a27:	75 e6                                           	jne    1b0a0f <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x10ef>
  1b0a29:	c6 43 45 00                                     	movb   $0x0,0x45(%rbx)
  1b0a2d:	80 fa 01                                        	cmp    $0x1,%dl
  1b0a30:	75 b7                                           	jne    1b09e9 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x10c9>
  1b0a32:	41 80 fe ff                                     	cmp    $0xff,%r14b
  1b0a36:	74 2e                                           	je     1b0a66 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x1146>
  1b0a38:	81 fd 00 00 00 08                               	cmp    $0x8000000,%ebp
  1b0a3e:	72 09                                           	jb     1b0a49 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x1129>
  1b0a40:	41 fe c6                                        	inc    %r14b
  1b0a43:	41 80 fe ff                                     	cmp    $0xff,%r14b
  1b0a47:	74 5b                                           	je     1b0aa4 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x1184>
  1b0a49:	4d 8b 7d 10                                     	mov    0x10(%r13),%r15
  1b0a4d:	4d 3b 7d 00                                     	cmp    0x0(%r13),%r15
  1b0a51:	75 87                                           	jne    1b09da <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x10ba>
  1b0a53:	4c 89 ef                                        	mov    %r13,%rdi
  1b0a56:	ff 15 94 d3 0b 00                               	call   *0xbd394(%rip)        # 26ddf0 <_DYNAMIC+0x270>
  1b0a5c:	48 8b 7c 24 08                                  	mov    0x8(%rsp),%rdi
  1b0a61:	e9 74 ff ff ff                                  	jmp    1b09da <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x10ba>
  1b0a66:	4d 8b 75 10                                     	mov    0x10(%r13),%r14
  1b0a6a:	4d 3b 75 00                                     	cmp    0x0(%r13),%r14
  1b0a6e:	75 0e                                           	jne    1b0a7e <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x115e>
  1b0a70:	4c 89 ef                                        	mov    %r13,%rdi
  1b0a73:	ff 15 77 d3 0b 00                               	call   *0xbd377(%rip)        # 26ddf0 <_DYNAMIC+0x270>
  1b0a79:	48 8b 7c 24 08                                  	mov    0x8(%rsp),%rdi
  1b0a7e:	49 8b 45 08                                     	mov    0x8(%r13),%rax
  1b0a82:	42 c6 04 30 ff                                  	movb   $0xff,(%rax,%r14,1)
  1b0a87:	49 ff c6                                        	inc    %r14
  1b0a8a:	4d 89 75 10                                     	mov    %r14,0x10(%r13)
  1b0a8e:	41 89 ee                                        	mov    %ebp,%r14d
  1b0a91:	41 c1 ee 14                                     	shr    $0x14,%r14d
  1b0a95:	b9 07 00 00 00                                  	mov    $0x7,%ecx
  1b0a9a:	b8 fe ff 0f 00                                  	mov    $0xffffe,%eax
  1b0a9f:	e9 56 ff ff ff                                  	jmp    1b09fa <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x10da>
  1b0aa4:	41 89 ee                                        	mov    %ebp,%r14d
  1b0aa7:	41 81 e6 fe ff ff 07                            	and    $0x7fffffe,%r14d
  1b0aae:	44 89 73 14                                     	mov    %r14d,0x14(%rbx)
  1b0ab2:	4d 8b 7d 10                                     	mov    0x10(%r13),%r15
  1b0ab6:	4d 3b 7d 00                                     	cmp    0x0(%r13),%r15
  1b0aba:	75 09                                           	jne    1b0ac5 <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x11a5>
  1b0abc:	4c 89 ef                                        	mov    %r13,%rdi
  1b0abf:	ff 15 2b d3 0b 00                               	call   *0xbd32b(%rip)        # 26ddf0 <_DYNAMIC+0x270>
  1b0ac5:	49 8b 45 08                                     	mov    0x8(%r13),%rax
  1b0ac9:	42 c6 04 38 ff                                  	movb   $0xff,(%rax,%r15,1)
  1b0ace:	49 ff c7                                        	inc    %r15
  1b0ad1:	4d 89 7d 10                                     	mov    %r15,0x10(%r13)
  1b0ad5:	41 c1 ee 14                                     	shr    $0x14,%r14d
  1b0ad9:	b9 07 00 00 00                                  	mov    $0x7,%ecx
  1b0ade:	b8 fe ff 0f 00                                  	mov    $0xffffe,%eax
  1b0ae3:	48 8b 7c 24 08                                  	mov    0x8(%rsp),%rdi
  1b0ae8:	e9 0d ff ff ff                                  	jmp    1b09fa <emuella_j2k_tier1::cleanup_pass_encode::<false>+0x10da>
  1b0aed:	48 8b 44 24 78                                  	mov    0x78(%rsp),%rax
  1b0af2:	c6 00 ff                                        	movb   $0xff,(%rax)
  1b0af5:	48 81 c4 c8 00 00 00                            	add    $0xc8,%rsp
  1b0afc:	5b                                              	pop    %rbx
  1b0afd:	41 5c                                           	pop    %r12
  1b0aff:	41 5d                                           	pop    %r13
  1b0b01:	41 5e                                           	pop    %r14
  1b0b03:	41 5f                                           	pop    %r15
  1b0b05:	5d                                              	pop    %rbp
  1b0b06:	c3                                              	ret
  1b0b07:	48 8d 15 22 8d 0b 00                            	lea    0xb8d22(%rip),%rdx        # 269830 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x2888>
  1b0b0e:	4c 89 c7                                        	mov    %r8,%rdi
  1b0b11:	4c 89 e6                                        	mov    %r12,%rsi
  1b0b14:	ff 15 8e d2 0b 00                               	call   *0xbd28e(%rip)        # 26dda8 <_DYNAMIC+0x228>
  1b0b1a:	48 8d 15 bf 94 0b 00                            	lea    0xb94bf(%rip),%rdx        # 269fe0 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x3038>
  1b0b21:	be 13 00 00 00                                  	mov    $0x13,%esi
  1b0b26:	ff 15 7c d2 0b 00                               	call   *0xbd27c(%rip)        # 26dda8 <_DYNAMIC+0x228>
  1b0b2c:	48 8d 15 c5 94 0b 00                            	lea    0xb94c5(%rip),%rdx        # 269ff8 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x3050>
  1b0b33:	be 2f 00 00 00                                  	mov    $0x2f,%esi
  1b0b38:	48 89 c7                                        	mov    %rax,%rdi
  1b0b3b:	ff 15 67 d2 0b 00                               	call   *0xbd267(%rip)        # 26dda8 <_DYNAMIC+0x228>
  1b0b41:	48 8d 15 70 8c 0b 00                            	lea    0xb8c70(%rip),%rdx        # 2697b8 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x2810>
  1b0b48:	4c 89 c7                                        	mov    %r8,%rdi
  1b0b4b:	48 8b 74 24 28                                  	mov    0x28(%rsp),%rsi
  1b0b50:	ff 15 52 d2 0b 00                               	call   *0xbd252(%rip)        # 26dda8 <_DYNAMIC+0x228>
  1b0b56:	4c 89 c7                                        	mov    %r8,%rdi
  1b0b59:	48 8d 15 d0 8c 0b 00                            	lea    0xb8cd0(%rip),%rdx        # 269830 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x2888>
  1b0b60:	4c 89 e6                                        	mov    %r12,%rsi
  1b0b63:	ff 15 3f d2 0b 00                               	call   *0xbd23f(%rip)        # 26dda8 <_DYNAMIC+0x228>
  1b0b69:	48 89 d7                                        	mov    %rdx,%rdi
  1b0b6c:	48 8d 15 bd 8c 0b 00                            	lea    0xb8cbd(%rip),%rdx        # 269830 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x2888>
  1b0b73:	4c 89 e6                                        	mov    %r12,%rsi
  1b0b76:	ff 15 2c d2 0b 00                               	call   *0xbd22c(%rip)        # 26dda8 <_DYNAMIC+0x228>
  1b0b7c:	4c 89 ff                                        	mov    %r15,%rdi
  1b0b7f:	48 8d 15 aa 8c 0b 00                            	lea    0xb8caa(%rip),%rdx        # 269830 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x2888>
  1b0b86:	4c 89 e6                                        	mov    %r12,%rsi
  1b0b89:	ff 15 19 d2 0b 00                               	call   *0xbd219(%rip)        # 26dda8 <_DYNAMIC+0x228>
  1b0b8f:	48 8d 15 9a 8c 0b 00                            	lea    0xb8c9a(%rip),%rdx        # 269830 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x2888>
  1b0b96:	4c 89 ef                                        	mov    %r13,%rdi
  1b0b99:	4c 89 e6                                        	mov    %r12,%rsi
  1b0b9c:	ff 15 06 d2 0b 00                               	call   *0xbd206(%rip)        # 26dda8 <_DYNAMIC+0x228>
  1b0ba2:	48 89 cf                                        	mov    %rcx,%rdi
  1b0ba5:	48 8d 15 84 8c 0b 00                            	lea    0xb8c84(%rip),%rdx        # 269830 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x2888>
  1b0bac:	4c 89 e6                                        	mov    %r12,%rsi
  1b0baf:	ff 15 f3 d1 0b 00                               	call   *0xbd1f3(%rip)        # 26dda8 <_DYNAMIC+0x228>
  1b0bb5:	48 8d 15 fc 8b 0b 00                            	lea    0xb8bfc(%rip),%rdx        # 2697b8 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x2810>
  1b0bbc:	4c 89 ef                                        	mov    %r13,%rdi
  1b0bbf:	48 8b 74 24 28                                  	mov    0x28(%rsp),%rsi
  1b0bc4:	ff 15 de d1 0b 00                               	call   *0xbd1de(%rip)        # 26dda8 <_DYNAMIC+0x228>
  1b0bca:	48 8d 15 27 94 0b 00                            	lea    0xb9427(%rip),%rdx        # 269ff8 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x3050>
  1b0bd1:	be 2f 00 00 00                                  	mov    $0x2f,%esi
  1b0bd6:	ff 15 cc d1 0b 00                               	call   *0xbd1cc(%rip)        # 26dda8 <_DYNAMIC+0x228>
  1b0bdc:	4c 89 cd                                        	mov    %r9,%rbp
  1b0bdf:	48 8d 15 d2 8b 0b 00                            	lea    0xb8bd2(%rip),%rdx        # 2697b8 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x2810>
  1b0be6:	48 89 ef                                        	mov    %rbp,%rdi
  1b0be9:	48 8b 74 24 28                                  	mov    0x28(%rsp),%rsi
  1b0bee:	ff 15 b4 d1 0b 00                               	call   *0xbd1b4(%rip)        # 26dda8 <_DYNAMIC+0x228>
  1b0bf4:	4c 89 fd                                        	mov    %r15,%rbp
  1b0bf7:	48 8d 15 ba 8b 0b 00                            	lea    0xb8bba(%rip),%rdx        # 2697b8 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x2810>
  1b0bfe:	48 89 ef                                        	mov    %rbp,%rdi
  1b0c01:	48 8b 74 24 28                                  	mov    0x28(%rsp),%rsi
  1b0c06:	ff 15 9c d1 0b 00                               	call   *0xbd19c(%rip)        # 26dda8 <_DYNAMIC+0x228>
  1b0c0c:	48 89 d5                                        	mov    %rdx,%rbp
  1b0c0f:	48 8d 15 a2 8b 0b 00                            	lea    0xb8ba2(%rip),%rdx        # 2697b8 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x2810>
  1b0c16:	48 89 ef                                        	mov    %rbp,%rdi
  1b0c19:	48 8b 74 24 28                                  	mov    0x28(%rsp),%rsi
  1b0c1e:	ff 15 84 d1 0b 00                               	call   *0xbd184(%rip)        # 26dda8 <_DYNAMIC+0x228>
  1b0c24:	48 8d 15 a5 8b 0b 00                            	lea    0xb8ba5(%rip),%rdx        # 2697d0 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x2828>
  1b0c2b:	4c 89 ef                                        	mov    %r13,%rdi
  1b0c2e:	4c 89 e6                                        	mov    %r12,%rsi
  1b0c31:	ff 15 71 d1 0b 00                               	call   *0xbd171(%rip)        # 26dda8 <_DYNAMIC+0x228>
