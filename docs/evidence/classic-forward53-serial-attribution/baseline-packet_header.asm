Disassembly of section .text:

00000000000e3fe0 <emuella_j2k_codestream::write_component_packet_header>:
   e3fe0:	55                                              	push   %rbp
   e3fe1:	41 57                                           	push   %r15
   e3fe3:	41 56                                           	push   %r14
   e3fe5:	41 55                                           	push   %r13
   e3fe7:	41 54                                           	push   %r12
   e3fe9:	53                                              	push   %rbx
   e3fea:	48 81 ec b8 01 00 00                            	sub    $0x1b8,%rsp
   e3ff1:	4c 89 8c 24 b8 00 00 00                         	mov    %r9,0xb8(%rsp)
   e3ff9:	89 cd                                           	mov    %ecx,%ebp
   e3ffb:	41 89 d4                                        	mov    %edx,%r12d
   e3ffe:	48 89 fb                                        	mov    %rdi,%rbx
   e4001:	41 0f b7 c4                                     	movzwl %r12w,%eax
   e4005:	44 0f b7 ed                                     	movzwl %bp,%r13d
   e4009:	4c 0f af e8                                     	imul   %rax,%r13
   e400d:	4d 85 ed                                        	test   %r13,%r13
   e4010:	0f 84 09 06 00 00                               	je     e461f <emuella_j2k_codestream::write_component_packet_header+0x63f>
   e4016:	4d 89 c6                                        	mov    %r8,%r14
   e4019:	49 89 f7                                        	mov    %rsi,%r15
   e401c:	4a 8d 04 ad 00 00 00 00                         	lea    0x0(,%r13,4),%rax
   e4024:	48 8d 3c 40                                     	lea    (%rax,%rax,2),%rdi
   e4028:	48 89 7c 24 20                                  	mov    %rdi,0x20(%rsp)
   e402d:	ff 15 b5 9d 18 00                               	call   *0x189db5(%rip)        # 26dde8 <malloc@GLIBC_2.2.5>
   e4033:	48 85 c0                                        	test   %rax,%rax
   e4036:	0f 84 c1 10 00 00                               	je     e50fd <emuella_j2k_codestream::write_component_packet_header+0x111d>
   e403c:	44 89 a4 24 e8 00 00 00                         	mov    %r12d,0xe8(%rsp)
   e4044:	89 ac 24 ec 00 00 00                            	mov    %ebp,0xec(%rsp)
   e404b:	4c 89 bc 24 b0 00 00 00                         	mov    %r15,0xb0(%rsp)
   e4053:	4c 89 b4 24 a8 00 00 00                         	mov    %r14,0xa8(%rsp)
   e405b:	48 89 5c 24 78                                  	mov    %rbx,0x78(%rsp)
   e4060:	48 8b 94 24 b8 00 00 00                         	mov    0xb8(%rsp),%rdx
   e4068:	48 89 d1                                        	mov    %rdx,%rcx
   e406b:	48 c1 e1 05                                     	shl    $0x5,%rcx
   e406f:	48 89 8c 24 c8 00 00 00                         	mov    %rcx,0xc8(%rsp)
   e4077:	4c 89 ac 24 f0 00 00 00                         	mov    %r13,0xf0(%rsp)
   e407f:	4c 89 ac 24 90 00 00 00                         	mov    %r13,0x90(%rsp)
   e4087:	48 89 84 24 98 00 00 00                         	mov    %rax,0x98(%rsp)
   e408f:	48 c7 84 24 a0 00 00 00 00 00 00 00             	movq   $0x0,0xa0(%rsp)
   e409b:	48 85 d2                                        	test   %rdx,%rdx
   e409e:	0f 84 87 00 00 00                               	je     e412b <emuella_j2k_codestream::write_component_packet_header+0x14b>
   e40a4:	41 bf 08 00 00 00                               	mov    $0x8,%r15d
   e40aa:	45 31 f6                                        	xor    %r14d,%r14d
   e40ad:	48 8d 9c 24 90 00 00 00                         	lea    0x90(%rsp),%rbx
   e40b5:	4c 8b 25 64 a3 18 00                            	mov    0x18a364(%rip),%r12        # 26e420 <_DYNAMIC+0x8a0>
   e40bc:	45 31 ed                                        	xor    %r13d,%r13d
   e40bf:	eb 42                                           	jmp    e4103 <emuella_j2k_codestream::write_component_packet_header+0x123>
   e40c1:	66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00    	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   e40d0:	83 f5 01                                        	xor    $0x1,%ebp
   e40d3:	42 c7 44 38 f8 00 00 00 00                      	movl   $0x0,-0x8(%rax,%r15,1)
   e40dc:	42 89 6c 38 fc                                  	mov    %ebp,-0x4(%rax,%r15,1)
   e40e1:	42 c6 04 38 00                                  	movb   $0x0,(%rax,%r15,1)
   e40e6:	49 ff c6                                        	inc    %r14
   e40e9:	4c 89 b4 24 a0 00 00 00                         	mov    %r14,0xa0(%rsp)
   e40f1:	49 83 c7 0c                                     	add    $0xc,%r15
   e40f5:	49 83 c5 20                                     	add    $0x20,%r13
   e40f9:	4c 39 ac 24 c8 00 00 00                         	cmp    %r13,0xc8(%rsp)
   e4101:	74 2b                                           	je     e412e <emuella_j2k_codestream::write_component_packet_header+0x14e>
   e4103:	48 8b 8c 24 a8 00 00 00                         	mov    0xa8(%rsp),%rcx
   e410b:	42 0f b6 6c 29 1b                               	movzbl 0x1b(%rcx,%r13,1),%ebp
   e4111:	4c 3b b4 24 90 00 00 00                         	cmp    0x90(%rsp),%r14
   e4119:	75 b5                                           	jne    e40d0 <emuella_j2k_codestream::write_component_packet_header+0xf0>
   e411b:	48 89 df                                        	mov    %rbx,%rdi
   e411e:	41 ff d4                                        	call   *%r12
   e4121:	48 8b 84 24 98 00 00 00                         	mov    0x98(%rsp),%rax
   e4129:	eb a5                                           	jmp    e40d0 <emuella_j2k_codestream::write_component_packet_header+0xf0>
   e412b:	45 31 f6                                        	xor    %r14d,%r14d
   e412e:	4c 8b a4 24 f0 00 00 00                         	mov    0xf0(%rsp),%r12
   e4136:	4d 39 e6                                        	cmp    %r12,%r14
   e4139:	0f 85 c2 04 00 00                               	jne    e4601 <emuella_j2k_codestream::write_component_packet_header+0x621>
   e413f:	48 c7 44 24 60 00 00 00 00                      	movq   $0x0,0x60(%rsp)
   e4148:	48 c7 44 24 68 08 00 00 00                      	movq   $0x8,0x68(%rsp)
   e4151:	48 c7 44 24 70 00 00 00 00                      	movq   $0x0,0x70(%rsp)
   e415a:	4c 8b bc 24 90 00 00 00                         	mov    0x90(%rsp),%r15
   e4162:	4c 8b b4 24 98 00 00 00                         	mov    0x98(%rsp),%r14
   e416a:	48 8d 7c 24 60                                  	lea    0x60(%rsp),%rdi
   e416f:	ff 15 eb a0 18 00                               	call   *0x18a0eb(%rip)        # 26e260 <_DYNAMIC+0x6e0>
   e4175:	48 8b 84 24 a8 00 00 00                         	mov    0xa8(%rsp),%rax
   e417d:	48 8b 8c 24 c8 00 00 00                         	mov    0xc8(%rsp),%rcx
   e4185:	48 01 c8                                        	add    %rcx,%rax
   e4188:	48 89 84 24 58 01 00 00                         	mov    %rax,0x158(%rsp)
   e4190:	48 8b 5c 24 68                                  	mov    0x68(%rsp),%rbx
   e4195:	4c 89 3b                                        	mov    %r15,(%rbx)
   e4198:	4c 89 73 08                                     	mov    %r14,0x8(%rbx)
   e419c:	4c 89 63 10                                     	mov    %r12,0x10(%rbx)
   e41a0:	8b 84 24 e8 00 00 00                            	mov    0xe8(%rsp),%eax
   e41a7:	66 89 43 18                                     	mov    %ax,0x18(%rbx)
   e41ab:	8b 84 24 ec 00 00 00                            	mov    0xec(%rsp),%eax
   e41b2:	66 89 43 1a                                     	mov    %ax,0x1a(%rbx)
   e41b6:	48 c7 44 24 70 01 00 00 00                      	movq   $0x1,0x70(%rsp)
   e41bf:	41 be 01 00 00 00                               	mov    $0x1,%r14d
   e41c5:	4c 8b 25 1c 9c 18 00                            	mov    0x189c1c(%rip),%r12        # 26dde8 <malloc@GLIBC_2.2.5>
   e41cc:	eb 48                                           	jmp    e4216 <emuella_j2k_codestream::write_component_packet_header+0x236>
   e41ce:	66 90                                           	xchg   %ax,%ax
   e41d0:	48 8d 7c 24 60                                  	lea    0x60(%rsp),%rdi
   e41d5:	ff 15 85 a0 18 00                               	call   *0x18a085(%rip)        # 26e260 <_DYNAMIC+0x6e0>
   e41db:	48 8b 5c 24 68                                  	mov    0x68(%rsp),%rbx
   e41e0:	4c 89 3c 2b                                     	mov    %r15,(%rbx,%rbp,1)
   e41e4:	48 8b 44 24 18                                  	mov    0x18(%rsp),%rax
   e41e9:	48 89 44 2b 08                                  	mov    %rax,0x8(%rbx,%rbp,1)
   e41ee:	4c 89 6c 2b 10                                  	mov    %r13,0x10(%rbx,%rbp,1)
   e41f3:	8b 84 24 c0 00 00 00                            	mov    0xc0(%rsp),%eax
   e41fa:	66 89 44 2b 18                                  	mov    %ax,0x18(%rbx,%rbp,1)
   e41ff:	8b 44 24 2c                                     	mov    0x2c(%rsp),%eax
   e4203:	66 89 44 2b 1a                                  	mov    %ax,0x1a(%rbx,%rbp,1)
   e4208:	49 ff c6                                        	inc    %r14
   e420b:	4c 89 74 24 70                                  	mov    %r14,0x70(%rsp)
   e4210:	0f 84 0a 05 00 00                               	je     e4720 <emuella_j2k_codestream::write_component_packet_header+0x740>
   e4216:	4c 89 f5                                        	mov    %r14,%rbp
   e4219:	48 c1 e5 05                                     	shl    $0x5,%rbp
   e421d:	0f b7 54 2b f8                                  	movzwl -0x8(%rbx,%rbp,1),%edx
   e4222:	89 d0                                           	mov    %edx,%eax
   e4224:	83 f0 01                                        	xor    $0x1,%eax
   e4227:	0f b7 74 2b fa                                  	movzwl -0x6(%rbx,%rbp,1),%esi
   e422c:	89 f1                                           	mov    %esi,%ecx
   e422e:	83 f1 01                                        	xor    $0x1,%ecx
   e4231:	66 09 c1                                        	or     %ax,%cx
   e4234:	0f 84 16 04 00 00                               	je     e4650 <emuella_j2k_codestream::write_component_packet_header+0x670>
   e423a:	89 d0                                           	mov    %edx,%eax
   e423c:	d1 e8                                           	shr    $1,%eax
   e423e:	48 89 54 24 18                                  	mov    %rdx,0x18(%rsp)
   e4243:	89 d1                                           	mov    %edx,%ecx
   e4245:	29 c1                                           	sub    %eax,%ecx
   e4247:	0f b7 c6                                        	movzwl %si,%eax
   e424a:	d1 e8                                           	shr    $1,%eax
   e424c:	89 b4 24 f8 00 00 00                            	mov    %esi,0xf8(%rsp)
   e4253:	89 f2                                           	mov    %esi,%edx
   e4255:	29 c2                                           	sub    %eax,%edx
   e4257:	89 8c 24 c0 00 00 00                            	mov    %ecx,0xc0(%rsp)
   e425e:	0f b7 c1                                        	movzwl %cx,%eax
   e4261:	89 54 24 2c                                     	mov    %edx,0x2c(%rsp)
   e4265:	44 0f b7 fa                                     	movzwl %dx,%r15d
   e4269:	4c 0f af f8                                     	imul   %rax,%r15
   e426d:	4d 85 ff                                        	test   %r15,%r15
   e4270:	74 2e                                           	je     e42a0 <emuella_j2k_codestream::write_component_packet_header+0x2c0>
   e4272:	4a 8d 04 bd 00 00 00 00                         	lea    0x0(,%r15,4),%rax
   e427a:	4c 8d 2c 40                                     	lea    (%rax,%rax,2),%r13
   e427e:	4c 89 ef                                        	mov    %r13,%rdi
   e4281:	41 ff d4                                        	call   *%r12
   e4284:	48 85 c0                                        	test   %rax,%rax
   e4287:	0f 84 54 0e 00 00                               	je     e50e1 <emuella_j2k_codestream::write_component_packet_header+0x1101>
   e428d:	49 89 c2                                        	mov    %rax,%r10
   e4290:	eb 14                                           	jmp    e42a6 <emuella_j2k_codestream::write_component_packet_header+0x2c6>
   e4292:	66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00       	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   e42a0:	41 ba 04 00 00 00                               	mov    $0x4,%r10d
   e42a6:	4c 89 7c 24 30                                  	mov    %r15,0x30(%rsp)
   e42ab:	4c 89 54 24 38                                  	mov    %r10,0x38(%rsp)
   e42b0:	48 c7 44 24 40 00 00 00 00                      	movq   $0x0,0x40(%rsp)
   e42b9:	66 83 bc 24 f8 00 00 00 00                      	cmpw   $0x0,0xf8(%rsp)
   e42c2:	0f 84 e8 02 00 00                               	je     e45b0 <emuella_j2k_codestream::write_component_packet_header+0x5d0>
   e42c8:	48 8b 54 24 18                                  	mov    0x18(%rsp),%rdx
   e42cd:	48 85 d2                                        	test   %rdx,%rdx
   e42d0:	0f 84 da 02 00 00                               	je     e45b0 <emuella_j2k_codestream::write_component_packet_header+0x5d0>
   e42d6:	48 89 ac 24 10 01 00 00                         	mov    %rbp,0x110(%rsp)
   e42de:	48 01 eb                                        	add    %rbp,%rbx
   e42e1:	8b 84 24 c0 00 00 00                            	mov    0xc0(%rsp),%eax
   e42e8:	66 83 f8 02                                     	cmp    $0x2,%ax
   e42ec:	89 c1                                           	mov    %eax,%ecx
   e42ee:	b8 01 00 00 00                                  	mov    $0x1,%eax
   e42f3:	0f 42 c8                                        	cmovb  %eax,%ecx
   e42f6:	89 8c 24 d8 00 00 00                            	mov    %ecx,0xd8(%rsp)
   e42fd:	8b 4c 24 2c                                     	mov    0x2c(%rsp),%ecx
   e4301:	66 83 f9 02                                     	cmp    $0x2,%cx
   e4305:	0f 42 c8                                        	cmovb  %eax,%ecx
   e4308:	89 8c 24 8c 00 00 00                            	mov    %ecx,0x8c(%rsp)
   e430f:	8d 04 12                                        	lea    (%rdx,%rdx,1),%eax
   e4312:	48 89 84 24 20 01 00 00                         	mov    %rax,0x120(%rsp)
   e431a:	8d 04 d5 00 00 00 00                            	lea    0x0(,%rdx,8),%eax
   e4321:	48 8d 04 40                                     	lea    (%rax,%rax,2),%rax
   e4325:	48 89 84 24 18 01 00 00                         	mov    %rax,0x118(%rsp)
   e432d:	8d 04 95 00 00 00 00                            	lea    0x0(,%rdx,4),%eax
   e4334:	48 8d 04 40                                     	lea    (%rax,%rax,2),%rax
   e4338:	48 89 84 24 80 00 00 00                         	mov    %rax,0x80(%rsp)
   e4340:	66 b8 02 00                                     	mov    $0x2,%ax
   e4344:	41 b8 04 00 00 00                               	mov    $0x4,%r8d
   e434a:	45 31 c9                                        	xor    %r9d,%r9d
   e434d:	31 ff                                           	xor    %edi,%edi
   e434f:	48 c7 84 24 d0 00 00 00 00 00 00 00             	movq   $0x0,0xd0(%rsp)
   e435b:	31 d2                                           	xor    %edx,%edx
   e435d:	eb 4f                                           	jmp    e43ae <emuella_j2k_codestream::write_component_packet_header+0x3ce>
   e435f:	90                                              	nop
   e4360:	48 8b 94 24 28 01 00 00                         	mov    0x128(%rsp),%rdx
   e4368:	ff c2                                           	inc    %edx
   e436a:	48 83 84 24 d0 00 00 00 02                      	addq   $0x2,0xd0(%rsp)
   e4373:	8b 84 24 00 01 00 00                            	mov    0x100(%rsp),%eax
   e437a:	83 c0 02                                        	add    $0x2,%eax
   e437d:	4c 8b 8c 24 30 01 00 00                         	mov    0x130(%rsp),%r9
   e4385:	4c 03 8c 24 20 01 00 00                         	add    0x120(%rsp),%r9
   e438d:	4c 8b 84 24 38 01 00 00                         	mov    0x138(%rsp),%r8
   e4395:	4c 03 84 24 18 01 00 00                         	add    0x118(%rsp),%r8
   e439d:	66 3b 94 24 8c 00 00 00                         	cmp    0x8c(%rsp),%dx
   e43a5:	48 89 cf                                        	mov    %rcx,%rdi
   e43a8:	0f 84 22 02 00 00                               	je     e45d0 <emuella_j2k_codestream::write_component_packet_header+0x5f0>
   e43ae:	8b b4 24 f8 00 00 00                            	mov    0xf8(%rsp),%esi
   e43b5:	66 39 c6                                        	cmp    %ax,%si
   e43b8:	89 84 24 00 01 00 00                            	mov    %eax,0x100(%rsp)
   e43bf:	0f 42 c6                                        	cmovb  %esi,%eax
   e43c2:	8d 0c 12                                        	lea    (%rdx,%rdx,1),%ecx
   e43c5:	48 89 94 24 28 01 00 00                         	mov    %rdx,0x128(%rsp)
   e43cd:	8d 14 55 02 00 00 00                            	lea    0x2(,%rdx,2),%edx
   e43d4:	66 39 d6                                        	cmp    %dx,%si
   e43d7:	0f 42 d6                                        	cmovb  %esi,%edx
   e43da:	66 39 d1                                        	cmp    %dx,%cx
   e43dd:	4c 89 84 24 38 01 00 00                         	mov    %r8,0x138(%rsp)
   e43e5:	4c 89 8c 24 30 01 00 00                         	mov    %r9,0x130(%rsp)
   e43ed:	0f 83 4d 01 00 00                               	jae    e4540 <emuella_j2k_codestream::write_component_packet_header+0x560>
   e43f3:	44 0f b7 e8                                     	movzwl %ax,%r13d
   e43f7:	66 b8 02 00                                     	mov    $0x2,%ax
   e43fb:	45 31 ff                                        	xor    %r15d,%r15d
   e43fe:	4c 89 84 24 08 01 00 00                         	mov    %r8,0x108(%rsp)
   e4406:	4c 89 8c 24 e0 00 00 00                         	mov    %r9,0xe0(%rsp)
   e440e:	48 89 f9                                        	mov    %rdi,%rcx
   e4411:	45 31 e4                                        	xor    %r12d,%r12d
   e4414:	eb 62                                           	jmp    e4478 <emuella_j2k_codestream::write_component_packet_header+0x498>
   e4416:	66 2e 0f 1f 84 00 00 00 00 00                   	cs nopw 0x0(%rax,%rax,1)
   e4420:	41 ff c4                                        	inc    %r12d
   e4423:	48 8b 54 24 38                                  	mov    0x38(%rsp),%rdx
   e4428:	48 8b 4c 24 10                                  	mov    0x10(%rsp),%rcx
   e442d:	48 8d 04 49                                     	lea    (%rcx,%rcx,2),%rax
   e4431:	c7 04 82 00 00 00 00                            	movl   $0x0,(%rdx,%rax,4)
   e4438:	89 6c 82 04                                     	mov    %ebp,0x4(%rdx,%rax,4)
   e443c:	49 89 d2                                        	mov    %rdx,%r10
   e443f:	c6 44 82 08 00                                  	movb   $0x0,0x8(%rdx,%rax,4)
   e4444:	48 ff c1                                        	inc    %rcx
   e4447:	48 89 4c 24 40                                  	mov    %rcx,0x40(%rsp)
   e444c:	49 83 c7 02                                     	add    $0x2,%r15
   e4450:	8b 44 24 28                                     	mov    0x28(%rsp),%eax
   e4454:	83 c0 02                                        	add    $0x2,%eax
   e4457:	48 83 84 24 e0 00 00 00 02                      	addq   $0x2,0xe0(%rsp)
   e4460:	48 83 84 24 08 01 00 00 18                      	addq   $0x18,0x108(%rsp)
   e4469:	66 44 3b a4 24 d8 00 00 00                      	cmp    0xd8(%rsp),%r12w
   e4472:	0f 84 e8 fe ff ff                               	je     e4360 <emuella_j2k_codestream::write_component_packet_header+0x380>
   e4478:	48 89 4c 24 10                                  	mov    %rcx,0x10(%rsp)
   e447d:	89 44 24 28                                     	mov    %eax,0x28(%rsp)
   e4481:	0f b7 c0                                        	movzwl %ax,%eax
   e4484:	48 8b 74 24 18                                  	mov    0x18(%rsp),%rsi
   e4489:	48 39 c6                                        	cmp    %rax,%rsi
   e448c:	48 0f 42 c6                                     	cmovb  %rsi,%rax
   e4490:	43 8d 0c 24                                     	lea    (%r12,%r12,1),%ecx
   e4494:	42 8d 14 65 02 00 00 00                         	lea    0x2(,%r12,2),%edx
   e449c:	66 39 d6                                        	cmp    %dx,%si
   e449f:	0f 43 f2                                        	cmovae %edx,%esi
   e44a2:	bd ff ff ff ff                                  	mov    $0xffffffff,%ebp
   e44a7:	66 39 f1                                        	cmp    %si,%cx
   e44aa:	73 6d                                           	jae    e4519 <emuella_j2k_codestream::write_component_packet_header+0x539>
   e44ac:	48 8b 73 f0                                     	mov    -0x10(%rbx),%rsi
   e44b0:	48 8b 8c 24 08 01 00 00                         	mov    0x108(%rsp),%rcx
   e44b8:	48 8b 94 24 e0 00 00 00                         	mov    0xe0(%rsp),%rdx
   e44c0:	4c 8b 84 24 d0 00 00 00                         	mov    0xd0(%rsp),%r8
   e44c8:	0f 1f 84 00 00 00 00 00                         	nopl   0x0(%rax,%rax,1)
   e44d0:	49 ff c0                                        	inc    %r8
   e44d3:	49 89 c9                                        	mov    %rcx,%r9
   e44d6:	48 89 d7                                        	mov    %rdx,%rdi
   e44d9:	49 89 c2                                        	mov    %rax,%r10
   e44dc:	0f 1f 40 00                                     	nopl   0x0(%rax)
   e44e0:	48 39 f7                                        	cmp    %rsi,%rdi
   e44e3:	0f 83 55 01 00 00                               	jae    e463e <emuella_j2k_codestream::write_component_packet_header+0x65e>
   e44e9:	4c 8b 5b e8                                     	mov    -0x18(%rbx),%r11
   e44ed:	47 8b 1c 0b                                     	mov    (%r11,%r9,1),%r11d
   e44f1:	41 39 eb                                        	cmp    %ebp,%r11d
   e44f4:	41 0f 42 eb                                     	cmovb  %r11d,%ebp
   e44f8:	49 ff ca                                        	dec    %r10
   e44fb:	48 ff c7                                        	inc    %rdi
   e44fe:	49 83 c1 0c                                     	add    $0xc,%r9
   e4502:	4d 39 d7                                        	cmp    %r10,%r15
   e4505:	75 d9                                           	jne    e44e0 <emuella_j2k_codestream::write_component_packet_header+0x500>
   e4507:	48 03 54 24 18                                  	add    0x18(%rsp),%rdx
   e450c:	48 03 8c 24 80 00 00 00                         	add    0x80(%rsp),%rcx
   e4514:	4d 39 e8                                        	cmp    %r13,%r8
   e4517:	75 b7                                           	jne    e44d0 <emuella_j2k_codestream::write_component_packet_header+0x4f0>
   e4519:	48 8b 44 24 10                                  	mov    0x10(%rsp),%rax
   e451e:	48 3b 44 24 30                                  	cmp    0x30(%rsp),%rax
   e4523:	0f 85 f7 fe ff ff                               	jne    e4420 <emuella_j2k_codestream::write_component_packet_header+0x440>
   e4529:	48 8d 7c 24 30                                  	lea    0x30(%rsp),%rdi
   e452e:	ff 15 ec 9e 18 00                               	call   *0x189eec(%rip)        # 26e420 <_DYNAMIC+0x8a0>
   e4534:	e9 e7 fe ff ff                                  	jmp    e4420 <emuella_j2k_codestream::write_component_packet_header+0x440>
   e4539:	0f 1f 80 00 00 00 00                            	nopl   0x0(%rax)
   e4540:	48 8d 04 7f                                     	lea    (%rdi,%rdi,2),%rax
   e4544:	4c 8d 3c 85 08 00 00 00                         	lea    0x8(,%rax,4),%r15
   e454c:	8b 84 24 d8 00 00 00                            	mov    0xd8(%rsp),%eax
   e4553:	89 c5                                           	mov    %eax,%ebp
   e4555:	48 89 f9                                        	mov    %rdi,%rcx
   e4558:	eb 35                                           	jmp    e458f <emuella_j2k_codestream::write_component_packet_header+0x5af>
   e455a:	66 0f 1f 44 00 00                               	nopw   0x0(%rax,%rax,1)
   e4560:	48 b8 00 00 00 00 ff ff ff ff                   	movabs $0xffffffff00000000,%rax
   e456a:	4a 89 44 39 f8                                  	mov    %rax,-0x8(%rcx,%r15,1)
   e456f:	49 89 ca                                        	mov    %rcx,%r10
   e4572:	42 c6 04 39 00                                  	movb   $0x0,(%rcx,%r15,1)
   e4577:	49 ff c4                                        	inc    %r12
   e457a:	4c 89 64 24 40                                  	mov    %r12,0x40(%rsp)
   e457f:	49 83 c7 0c                                     	add    $0xc,%r15
   e4583:	66 ff cd                                        	dec    %bp
   e4586:	4c 89 e1                                        	mov    %r12,%rcx
   e4589:	0f 84 d1 fd ff ff                               	je     e4360 <emuella_j2k_codestream::write_component_packet_header+0x380>
   e458f:	48 3b 4c 24 30                                  	cmp    0x30(%rsp),%rcx
   e4594:	49 89 cc                                        	mov    %rcx,%r12
   e4597:	4c 89 d1                                        	mov    %r10,%rcx
   e459a:	75 c4                                           	jne    e4560 <emuella_j2k_codestream::write_component_packet_header+0x580>
   e459c:	48 8d 7c 24 30                                  	lea    0x30(%rsp),%rdi
   e45a1:	ff 15 79 9e 18 00                               	call   *0x189e79(%rip)        # 26e420 <_DYNAMIC+0x8a0>
   e45a7:	48 8b 4c 24 38                                  	mov    0x38(%rsp),%rcx
   e45ac:	eb b2                                           	jmp    e4560 <emuella_j2k_codestream::write_component_packet_header+0x580>
   e45ae:	66 90                                           	xchg   %ax,%ax
   e45b0:	4c 89 54 24 18                                  	mov    %r10,0x18(%rsp)
   e45b5:	45 31 ed                                        	xor    %r13d,%r13d
   e45b8:	4c 3b 74 24 60                                  	cmp    0x60(%rsp),%r14
   e45bd:	0f 84 0d fc ff ff                               	je     e41d0 <emuella_j2k_codestream::write_component_packet_header+0x1f0>
   e45c3:	e9 13 fc ff ff                                  	jmp    e41db <emuella_j2k_codestream::write_component_packet_header+0x1fb>
   e45c8:	0f 1f 84 00 00 00 00 00                         	nopl   0x0(%rax,%rax,1)
   e45d0:	49 89 fd                                        	mov    %rdi,%r13
   e45d3:	4c 8b 7c 24 30                                  	mov    0x30(%rsp),%r15
   e45d8:	48 8b 44 24 38                                  	mov    0x38(%rsp),%rax
   e45dd:	48 89 44 24 18                                  	mov    %rax,0x18(%rsp)
   e45e2:	4c 8b 25 ff 97 18 00                            	mov    0x1897ff(%rip),%r12        # 26dde8 <malloc@GLIBC_2.2.5>
   e45e9:	48 8b ac 24 10 01 00 00                         	mov    0x110(%rsp),%rbp
   e45f1:	4c 3b 74 24 60                                  	cmp    0x60(%rsp),%r14
   e45f6:	0f 85 df fb ff ff                               	jne    e41db <emuella_j2k_codestream::write_component_packet_header+0x1fb>
   e45fc:	e9 cf fb ff ff                                  	jmp    e41d0 <emuella_j2k_codestream::write_component_packet_header+0x1f0>
   e4601:	48 83 bc 24 90 00 00 00 00                      	cmpq   $0x0,0x90(%rsp)
   e460a:	74 0e                                           	je     e461a <emuella_j2k_codestream::write_component_packet_header+0x63a>
   e460c:	48 8b bc 24 98 00 00 00                         	mov    0x98(%rsp),%rdi
   e4614:	ff 15 b6 97 18 00                               	call   *0x1897b6(%rip)        # 26ddd0 <free@GLIBC_2.2.5>
   e461a:	48 8b 5c 24 78                                  	mov    0x78(%rsp),%rbx
   e461f:	48 b8 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rax
   e4629:	48 89 03                                        	mov    %rax,(%rbx)
   e462c:	48 81 c4 b8 01 00 00                            	add    $0x1b8,%rsp
   e4633:	5b                                              	pop    %rbx
   e4634:	41 5c                                           	pop    %r12
   e4636:	41 5d                                           	pop    %r13
   e4638:	41 5e                                           	pop    %r14
   e463a:	41 5f                                           	pop    %r15
   e463c:	5d                                              	pop    %rbp
   e463d:	c3                                              	ret
   e463e:	48 8d 15 e3 2b 18 00                            	lea    0x182be3(%rip),%rdx        # 267228 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x280>
   e4645:	ff 15 5d 97 18 00                               	call   *0x18975d(%rip)        # 26dda8 <_DYNAMIC+0x228>
   e464b:	e9 db 0a 00 00                                  	jmp    e512b <emuella_j2k_codestream::write_component_packet_header+0x114b>
   e4650:	48 8b 6c 24 60                                  	mov    0x60(%rsp),%rbp
   e4655:	4c 8b 7c 24 20                                  	mov    0x20(%rsp),%r15
   e465a:	4c 89 ff                                        	mov    %r15,%rdi
   e465d:	ff 15 85 97 18 00                               	call   *0x189785(%rip)        # 26dde8 <malloc@GLIBC_2.2.5>
   e4663:	48 85 c0                                        	test   %rax,%rax
   e4666:	0f 84 b1 0a 00 00                               	je     e511d <emuella_j2k_codestream::write_component_packet_header+0x113d>
   e466c:	48 89 6c 24 20                                  	mov    %rbp,0x20(%rsp)
   e4671:	48 8b 8c 24 f0 00 00 00                         	mov    0xf0(%rsp),%rcx
   e4679:	48 89 8c 24 90 00 00 00                         	mov    %rcx,0x90(%rsp)
   e4681:	48 89 84 24 98 00 00 00                         	mov    %rax,0x98(%rsp)
   e4689:	48 c7 84 24 a0 00 00 00 00 00 00 00             	movq   $0x0,0xa0(%rsp)
   e4695:	48 83 bc 24 b8 00 00 00 00                      	cmpq   $0x0,0xb8(%rsp)
   e469e:	0f 84 90 00 00 00                               	je     e4734 <emuella_j2k_codestream::write_component_packet_header+0x754>
   e46a4:	41 bd 08 00 00 00                               	mov    $0x8,%r13d
   e46aa:	45 31 e4                                        	xor    %r12d,%r12d
   e46ad:	31 ed                                           	xor    %ebp,%ebp
   e46af:	eb 3f                                           	jmp    e46f0 <emuella_j2k_codestream::write_component_packet_header+0x710>
   e46b1:	66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00    	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   e46c0:	42 c7 44 28 f8 00 00 00 00                      	movl   $0x0,-0x8(%rax,%r13,1)
   e46c9:	46 89 7c 28 fc                                  	mov    %r15d,-0x4(%rax,%r13,1)
   e46ce:	42 c6 04 28 00                                  	movb   $0x0,(%rax,%r13,1)
   e46d3:	49 ff c4                                        	inc    %r12
   e46d6:	4c 89 a4 24 a0 00 00 00                         	mov    %r12,0xa0(%rsp)
   e46de:	49 83 c5 0c                                     	add    $0xc,%r13
   e46e2:	48 83 c5 20                                     	add    $0x20,%rbp
   e46e6:	48 39 ac 24 c8 00 00 00                         	cmp    %rbp,0xc8(%rsp)
   e46ee:	74 47                                           	je     e4737 <emuella_j2k_codestream::write_component_packet_header+0x757>
   e46f0:	48 8b 8c 24 a8 00 00 00                         	mov    0xa8(%rsp),%rcx
   e46f8:	44 0f b6 7c 29 1a                               	movzbl 0x1a(%rcx,%rbp,1),%r15d
   e46fe:	4c 3b a4 24 90 00 00 00                         	cmp    0x90(%rsp),%r12
   e4706:	75 b8                                           	jne    e46c0 <emuella_j2k_codestream::write_component_packet_header+0x6e0>
   e4708:	48 8d bc 24 90 00 00 00                         	lea    0x90(%rsp),%rdi
   e4710:	ff 15 0a 9d 18 00                               	call   *0x189d0a(%rip)        # 26e420 <_DYNAMIC+0x8a0>
   e4716:	48 8b 84 24 98 00 00 00                         	mov    0x98(%rsp),%rax
   e471e:	eb a0                                           	jmp    e46c0 <emuella_j2k_codestream::write_component_packet_header+0x6e0>
   e4720:	48 83 7c 24 60 00                               	cmpq   $0x0,0x60(%rsp)
   e4726:	0f 84 ee fe ff ff                               	je     e461a <emuella_j2k_codestream::write_component_packet_header+0x63a>
   e472c:	48 89 df                                        	mov    %rbx,%rdi
   e472f:	e9 e0 fe ff ff                                  	jmp    e4614 <emuella_j2k_codestream::write_component_packet_header+0x634>
   e4734:	45 31 e4                                        	xor    %r12d,%r12d
   e4737:	4c 3b a4 24 f0 00 00 00                         	cmp    0xf0(%rsp),%r12
   e473f:	0f 85 ba 04 00 00                               	jne    e4bff <emuella_j2k_codestream::write_component_packet_header+0xc1f>
   e4745:	48 c7 44 24 60 00 00 00 00                      	movq   $0x0,0x60(%rsp)
   e474e:	48 c7 44 24 68 08 00 00 00                      	movq   $0x8,0x68(%rsp)
   e4757:	48 c7 44 24 70 00 00 00 00                      	movq   $0x0,0x70(%rsp)
   e4760:	4c 8b ac 24 90 00 00 00                         	mov    0x90(%rsp),%r13
   e4768:	4c 8b bc 24 98 00 00 00                         	mov    0x98(%rsp),%r15
   e4770:	48 8d 7c 24 60                                  	lea    0x60(%rsp),%rdi
   e4775:	ff 15 e5 9a 18 00                               	call   *0x189ae5(%rip)        # 26e260 <_DYNAMIC+0x6e0>
   e477b:	48 8b 6c 24 68                                  	mov    0x68(%rsp),%rbp
   e4780:	4c 89 6d 00                                     	mov    %r13,0x0(%rbp)
   e4784:	4c 89 7d 08                                     	mov    %r15,0x8(%rbp)
   e4788:	48 8b 84 24 f0 00 00 00                         	mov    0xf0(%rsp),%rax
   e4790:	48 89 45 10                                     	mov    %rax,0x10(%rbp)
   e4794:	8b 84 24 e8 00 00 00                            	mov    0xe8(%rsp),%eax
   e479b:	66 89 45 18                                     	mov    %ax,0x18(%rbp)
   e479f:	8b 84 24 ec 00 00 00                            	mov    0xec(%rsp),%eax
   e47a6:	66 89 45 1a                                     	mov    %ax,0x1a(%rbp)
   e47aa:	48 c7 44 24 70 01 00 00 00                      	movq   $0x1,0x70(%rsp)
   e47b3:	41 bd 01 00 00 00                               	mov    $0x1,%r13d
   e47b9:	eb 4f                                           	jmp    e480a <emuella_j2k_codestream::write_component_packet_header+0x82a>
   e47bb:	48 8d 7c 24 60                                  	lea    0x60(%rsp),%rdi
   e47c0:	ff 15 9a 9a 18 00                               	call   *0x189a9a(%rip)        # 26e260 <_DYNAMIC+0x6e0>
   e47c6:	48 8b 6c 24 68                                  	mov    0x68(%rsp),%rbp
   e47cb:	48 8b 84 24 c0 00 00 00                         	mov    0xc0(%rsp),%rax
   e47d3:	4c 89 7c 05 00                                  	mov    %r15,0x0(%rbp,%rax,1)
   e47d8:	4c 89 64 05 08                                  	mov    %r12,0x8(%rbp,%rax,1)
   e47dd:	48 8b 4c 24 10                                  	mov    0x10(%rsp),%rcx
   e47e2:	48 89 4c 05 10                                  	mov    %rcx,0x10(%rbp,%rax,1)
   e47e7:	8b 4c 24 2c                                     	mov    0x2c(%rsp),%ecx
   e47eb:	66 89 4c 05 18                                  	mov    %cx,0x18(%rbp,%rax,1)
   e47f0:	8b 8c 24 b8 00 00 00                            	mov    0xb8(%rsp),%ecx
   e47f7:	66 89 4c 05 1a                                  	mov    %cx,0x1a(%rbp,%rax,1)
   e47fc:	49 ff c5                                        	inc    %r13
   e47ff:	4c 89 6c 24 70                                  	mov    %r13,0x70(%rsp)
   e4804:	0f 84 4a 06 00 00                               	je     e4e54 <emuella_j2k_codestream::write_component_packet_header+0xe74>
   e480a:	4c 89 e9                                        	mov    %r13,%rcx
   e480d:	48 c1 e1 05                                     	shl    $0x5,%rcx
   e4811:	0f b7 54 0d f8                                  	movzwl -0x8(%rbp,%rcx,1),%edx
   e4816:	89 d0                                           	mov    %edx,%eax
   e4818:	83 f0 01                                        	xor    $0x1,%eax
   e481b:	48 89 8c 24 c0 00 00 00                         	mov    %rcx,0xc0(%rsp)
   e4823:	0f b7 74 0d fa                                  	movzwl -0x6(%rbp,%rcx,1),%esi
   e4828:	89 f1                                           	mov    %esi,%ecx
   e482a:	83 f1 01                                        	xor    $0x1,%ecx
   e482d:	66 09 c1                                        	or     %ax,%cx
   e4830:	0f 84 02 04 00 00                               	je     e4c38 <emuella_j2k_codestream::write_component_packet_header+0xc58>
   e4836:	89 d0                                           	mov    %edx,%eax
   e4838:	d1 e8                                           	shr    $1,%eax
   e483a:	48 89 54 24 18                                  	mov    %rdx,0x18(%rsp)
   e483f:	89 d1                                           	mov    %edx,%ecx
   e4841:	29 c1                                           	sub    %eax,%ecx
   e4843:	0f b7 c6                                        	movzwl %si,%eax
   e4846:	d1 e8                                           	shr    $1,%eax
   e4848:	89 b4 24 8c 00 00 00                            	mov    %esi,0x8c(%rsp)
   e484f:	89 f2                                           	mov    %esi,%edx
   e4851:	29 c2                                           	sub    %eax,%edx
   e4853:	89 4c 24 2c                                     	mov    %ecx,0x2c(%rsp)
   e4857:	0f b7 c1                                        	movzwl %cx,%eax
   e485a:	89 94 24 b8 00 00 00                            	mov    %edx,0xb8(%rsp)
   e4861:	44 0f b7 fa                                     	movzwl %dx,%r15d
   e4865:	4c 0f af f8                                     	imul   %rax,%r15
   e4869:	4d 85 ff                                        	test   %r15,%r15
   e486c:	74 1f                                           	je     e488d <emuella_j2k_codestream::write_component_packet_header+0x8ad>
   e486e:	4a 8d 04 bd 00 00 00 00                         	lea    0x0(,%r15,4),%rax
   e4876:	4c 8d 24 40                                     	lea    (%rax,%rax,2),%r12
   e487a:	4c 89 e7                                        	mov    %r12,%rdi
   e487d:	ff 15 65 95 18 00                               	call   *0x189565(%rip)        # 26dde8 <malloc@GLIBC_2.2.5>
   e4883:	48 85 c0                                        	test   %rax,%rax
   e4886:	75 0a                                           	jne    e4892 <emuella_j2k_codestream::write_component_packet_header+0x8b2>
   e4888:	e9 80 08 00 00                                  	jmp    e510d <emuella_j2k_codestream::write_component_packet_header+0x112d>
   e488d:	b8 04 00 00 00                                  	mov    $0x4,%eax
   e4892:	4c 89 7c 24 30                                  	mov    %r15,0x30(%rsp)
   e4897:	48 89 44 24 38                                  	mov    %rax,0x38(%rsp)
   e489c:	48 c7 44 24 40 00 00 00 00                      	movq   $0x0,0x40(%rsp)
   e48a5:	66 83 bc 24 8c 00 00 00 00                      	cmpw   $0x0,0x8c(%rsp)
   e48ae:	0f 84 05 03 00 00                               	je     e4bb9 <emuella_j2k_codestream::write_component_packet_header+0xbd9>
   e48b4:	48 8b 54 24 18                                  	mov    0x18(%rsp),%rdx
   e48b9:	48 85 d2                                        	test   %rdx,%rdx
   e48bc:	0f 84 f7 02 00 00                               	je     e4bb9 <emuella_j2k_codestream::write_component_packet_header+0xbd9>
   e48c2:	49 89 c2                                        	mov    %rax,%r10
   e48c5:	4c 89 ac 24 10 01 00 00                         	mov    %r13,0x110(%rsp)
   e48cd:	48 03 ac 24 c0 00 00 00                         	add    0xc0(%rsp),%rbp
   e48d5:	8b 44 24 2c                                     	mov    0x2c(%rsp),%eax
   e48d9:	66 83 f8 02                                     	cmp    $0x2,%ax
   e48dd:	89 c1                                           	mov    %eax,%ecx
   e48df:	b8 01 00 00 00                                  	mov    $0x1,%eax
   e48e4:	0f 42 c8                                        	cmovb  %eax,%ecx
   e48e7:	89 8c 24 d0 00 00 00                            	mov    %ecx,0xd0(%rsp)
   e48ee:	8b 8c 24 b8 00 00 00                            	mov    0xb8(%rsp),%ecx
   e48f5:	66 83 f9 02                                     	cmp    $0x2,%cx
   e48f9:	0f 42 c8                                        	cmovb  %eax,%ecx
   e48fc:	89 8c 24 20 01 00 00                            	mov    %ecx,0x120(%rsp)
   e4903:	8d 04 12                                        	lea    (%rdx,%rdx,1),%eax
   e4906:	48 89 84 24 18 01 00 00                         	mov    %rax,0x118(%rsp)
   e490e:	8d 04 d5 00 00 00 00                            	lea    0x0(,%rdx,8),%eax
   e4915:	48 8d 04 40                                     	lea    (%rax,%rax,2),%rax
   e4919:	48 89 84 24 c8 00 00 00                         	mov    %rax,0xc8(%rsp)
   e4921:	8d 04 95 00 00 00 00                            	lea    0x0(,%rdx,4),%eax
   e4928:	48 8d 04 40                                     	lea    (%rax,%rax,2),%rax
   e492c:	48 89 84 24 80 00 00 00                         	mov    %rax,0x80(%rsp)
   e4934:	66 b8 02 00                                     	mov    $0x2,%ax
   e4938:	41 b8 04 00 00 00                               	mov    $0x4,%r8d
   e493e:	45 31 c9                                        	xor    %r9d,%r9d
   e4941:	31 ff                                           	xor    %edi,%edi
   e4943:	48 c7 84 24 00 01 00 00 00 00 00 00             	movq   $0x0,0x100(%rsp)
   e494f:	31 d2                                           	xor    %edx,%edx
   e4951:	eb 5b                                           	jmp    e49ae <emuella_j2k_codestream::write_component_packet_header+0x9ce>
   e4953:	66 66 66 66 2e 0f 1f 84 00 00 00 00 00          	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   e4960:	48 8b 94 24 f8 00 00 00                         	mov    0xf8(%rsp),%rdx
   e4968:	ff c2                                           	inc    %edx
   e496a:	48 83 84 24 00 01 00 00 02                      	addq   $0x2,0x100(%rsp)
   e4973:	8b 84 24 38 01 00 00                            	mov    0x138(%rsp),%eax
   e497a:	83 c0 02                                        	add    $0x2,%eax
   e497d:	4c 8b 8c 24 28 01 00 00                         	mov    0x128(%rsp),%r9
   e4985:	4c 03 8c 24 18 01 00 00                         	add    0x118(%rsp),%r9
   e498d:	4c 8b 84 24 30 01 00 00                         	mov    0x130(%rsp),%r8
   e4995:	4c 03 84 24 c8 00 00 00                         	add    0xc8(%rsp),%r8
   e499d:	66 3b 94 24 20 01 00 00                         	cmp    0x120(%rsp),%dx
   e49a5:	48 89 cf                                        	mov    %rcx,%rdi
   e49a8:	0f 84 27 02 00 00                               	je     e4bd5 <emuella_j2k_codestream::write_component_packet_header+0xbf5>
   e49ae:	8b b4 24 8c 00 00 00                            	mov    0x8c(%rsp),%esi
   e49b5:	66 39 c6                                        	cmp    %ax,%si
   e49b8:	89 84 24 38 01 00 00                            	mov    %eax,0x138(%rsp)
   e49bf:	0f 42 c6                                        	cmovb  %esi,%eax
   e49c2:	8d 0c 12                                        	lea    (%rdx,%rdx,1),%ecx
   e49c5:	48 89 94 24 f8 00 00 00                         	mov    %rdx,0xf8(%rsp)
   e49cd:	8d 14 55 02 00 00 00                            	lea    0x2(,%rdx,2),%edx
   e49d4:	66 39 d6                                        	cmp    %dx,%si
   e49d7:	0f 42 d6                                        	cmovb  %esi,%edx
   e49da:	66 39 d1                                        	cmp    %dx,%cx
   e49dd:	4c 89 84 24 30 01 00 00                         	mov    %r8,0x130(%rsp)
   e49e5:	4c 89 8c 24 28 01 00 00                         	mov    %r9,0x128(%rsp)
   e49ed:	0f 83 5d 01 00 00                               	jae    e4b50 <emuella_j2k_codestream::write_component_packet_header+0xb70>
   e49f3:	48 89 f9                                        	mov    %rdi,%rcx
   e49f6:	44 0f b7 e8                                     	movzwl %ax,%r13d
   e49fa:	66 b8 02 00                                     	mov    $0x2,%ax
   e49fe:	45 31 e4                                        	xor    %r12d,%r12d
   e4a01:	4c 89 84 24 e0 00 00 00                         	mov    %r8,0xe0(%rsp)
   e4a09:	4c 89 8c 24 d8 00 00 00                         	mov    %r9,0xd8(%rsp)
   e4a11:	31 d2                                           	xor    %edx,%edx
   e4a13:	eb 69                                           	jmp    e4a7e <emuella_j2k_codestream::write_component_packet_header+0xa9e>
   e4a15:	66 66 2e 0f 1f 84 00 00 00 00 00                	data16 cs nopw 0x0(%rax,%rax,1)
   e4a20:	48 8b 94 24 08 01 00 00                         	mov    0x108(%rsp),%rdx
   e4a28:	ff c2                                           	inc    %edx
   e4a2a:	4c 8b 54 24 38                                  	mov    0x38(%rsp),%r10
   e4a2f:	48 8b 4c 24 10                                  	mov    0x10(%rsp),%rcx
   e4a34:	48 8d 04 49                                     	lea    (%rcx,%rcx,2),%rax
   e4a38:	41 c7 04 82 00 00 00 00                         	movl   $0x0,(%r10,%rax,4)
   e4a40:	45 89 7c 82 04                                  	mov    %r15d,0x4(%r10,%rax,4)
   e4a45:	41 c6 44 82 08 00                               	movb   $0x0,0x8(%r10,%rax,4)
   e4a4b:	48 ff c1                                        	inc    %rcx
   e4a4e:	48 89 4c 24 40                                  	mov    %rcx,0x40(%rsp)
   e4a53:	49 83 c4 02                                     	add    $0x2,%r12
   e4a57:	8b 44 24 28                                     	mov    0x28(%rsp),%eax
   e4a5b:	83 c0 02                                        	add    $0x2,%eax
   e4a5e:	48 83 84 24 d8 00 00 00 02                      	addq   $0x2,0xd8(%rsp)
   e4a67:	48 83 84 24 e0 00 00 00 18                      	addq   $0x18,0xe0(%rsp)
   e4a70:	66 3b 94 24 d0 00 00 00                         	cmp    0xd0(%rsp),%dx
   e4a78:	0f 84 e2 fe ff ff                               	je     e4960 <emuella_j2k_codestream::write_component_packet_header+0x980>
   e4a7e:	48 89 4c 24 10                                  	mov    %rcx,0x10(%rsp)
   e4a83:	89 44 24 28                                     	mov    %eax,0x28(%rsp)
   e4a87:	0f b7 c0                                        	movzwl %ax,%eax
   e4a8a:	48 8b 74 24 18                                  	mov    0x18(%rsp),%rsi
   e4a8f:	48 39 c6                                        	cmp    %rax,%rsi
   e4a92:	48 0f 42 c6                                     	cmovb  %rsi,%rax
   e4a96:	8d 0c 12                                        	lea    (%rdx,%rdx,1),%ecx
   e4a99:	48 89 94 24 08 01 00 00                         	mov    %rdx,0x108(%rsp)
   e4aa1:	8d 14 55 02 00 00 00                            	lea    0x2(,%rdx,2),%edx
   e4aa8:	66 39 d6                                        	cmp    %dx,%si
   e4aab:	0f 43 f2                                        	cmovae %edx,%esi
   e4aae:	41 bf ff ff ff ff                               	mov    $0xffffffff,%r15d
   e4ab4:	66 39 f1                                        	cmp    %si,%cx
   e4ab7:	73 70                                           	jae    e4b29 <emuella_j2k_codestream::write_component_packet_header+0xb49>
   e4ab9:	48 8b 75 f0                                     	mov    -0x10(%rbp),%rsi
   e4abd:	48 8b 8c 24 e0 00 00 00                         	mov    0xe0(%rsp),%rcx
   e4ac5:	48 8b 94 24 d8 00 00 00                         	mov    0xd8(%rsp),%rdx
   e4acd:	4c 8b 84 24 00 01 00 00                         	mov    0x100(%rsp),%r8
   e4ad5:	66 66 2e 0f 1f 84 00 00 00 00 00                	data16 cs nopw 0x0(%rax,%rax,1)
   e4ae0:	49 ff c0                                        	inc    %r8
   e4ae3:	49 89 c9                                        	mov    %rcx,%r9
   e4ae6:	48 89 d7                                        	mov    %rdx,%rdi
   e4ae9:	49 89 c2                                        	mov    %rax,%r10
   e4aec:	0f 1f 40 00                                     	nopl   0x0(%rax)
   e4af0:	48 39 f7                                        	cmp    %rsi,%rdi
   e4af3:	0f 83 2d 01 00 00                               	jae    e4c26 <emuella_j2k_codestream::write_component_packet_header+0xc46>
   e4af9:	4c 8b 5d e8                                     	mov    -0x18(%rbp),%r11
   e4afd:	47 8b 1c 0b                                     	mov    (%r11,%r9,1),%r11d
   e4b01:	45 39 fb                                        	cmp    %r15d,%r11d
   e4b04:	45 0f 42 fb                                     	cmovb  %r11d,%r15d
   e4b08:	49 ff ca                                        	dec    %r10
   e4b0b:	48 ff c7                                        	inc    %rdi
   e4b0e:	49 83 c1 0c                                     	add    $0xc,%r9
   e4b12:	4d 39 d4                                        	cmp    %r10,%r12
   e4b15:	75 d9                                           	jne    e4af0 <emuella_j2k_codestream::write_component_packet_header+0xb10>
   e4b17:	48 03 54 24 18                                  	add    0x18(%rsp),%rdx
   e4b1c:	48 03 8c 24 80 00 00 00                         	add    0x80(%rsp),%rcx
   e4b24:	4d 39 e8                                        	cmp    %r13,%r8
   e4b27:	75 b7                                           	jne    e4ae0 <emuella_j2k_codestream::write_component_packet_header+0xb00>
   e4b29:	48 8b 44 24 10                                  	mov    0x10(%rsp),%rax
   e4b2e:	48 3b 44 24 30                                  	cmp    0x30(%rsp),%rax
   e4b33:	0f 85 e7 fe ff ff                               	jne    e4a20 <emuella_j2k_codestream::write_component_packet_header+0xa40>
   e4b39:	48 8d 7c 24 30                                  	lea    0x30(%rsp),%rdi
   e4b3e:	ff 15 dc 98 18 00                               	call   *0x1898dc(%rip)        # 26e420 <_DYNAMIC+0x8a0>
   e4b44:	e9 d7 fe ff ff                                  	jmp    e4a20 <emuella_j2k_codestream::write_component_packet_header+0xa40>
   e4b49:	0f 1f 80 00 00 00 00                            	nopl   0x0(%rax)
   e4b50:	48 8d 04 7f                                     	lea    (%rdi,%rdi,2),%rax
   e4b54:	4c 8d 3c 85 08 00 00 00                         	lea    0x8(,%rax,4),%r15
   e4b5c:	8b 84 24 d0 00 00 00                            	mov    0xd0(%rsp),%eax
   e4b63:	41 89 c4                                        	mov    %eax,%r12d
   e4b66:	48 89 f9                                        	mov    %rdi,%rcx
   e4b69:	eb 32                                           	jmp    e4b9d <emuella_j2k_codestream::write_component_packet_header+0xbbd>
   e4b6b:	0f 1f 44 00 00                                  	nopl   0x0(%rax,%rax,1)
   e4b70:	48 b8 00 00 00 00 ff ff ff ff                   	movabs $0xffffffff00000000,%rax
   e4b7a:	4b 89 44 3a f8                                  	mov    %rax,-0x8(%r10,%r15,1)
   e4b7f:	43 c6 04 3a 00                                  	movb   $0x0,(%r10,%r15,1)
   e4b84:	49 ff c5                                        	inc    %r13
   e4b87:	4c 89 6c 24 40                                  	mov    %r13,0x40(%rsp)
   e4b8c:	49 83 c7 0c                                     	add    $0xc,%r15
   e4b90:	66 41 ff cc                                     	dec    %r12w
   e4b94:	4c 89 e9                                        	mov    %r13,%rcx
   e4b97:	0f 84 c3 fd ff ff                               	je     e4960 <emuella_j2k_codestream::write_component_packet_header+0x980>
   e4b9d:	48 3b 4c 24 30                                  	cmp    0x30(%rsp),%rcx
   e4ba2:	49 89 cd                                        	mov    %rcx,%r13
   e4ba5:	75 c9                                           	jne    e4b70 <emuella_j2k_codestream::write_component_packet_header+0xb90>
   e4ba7:	48 8d 7c 24 30                                  	lea    0x30(%rsp),%rdi
   e4bac:	ff 15 6e 98 18 00                               	call   *0x18986e(%rip)        # 26e420 <_DYNAMIC+0x8a0>
   e4bb2:	4c 8b 54 24 38                                  	mov    0x38(%rsp),%r10
   e4bb7:	eb b7                                           	jmp    e4b70 <emuella_j2k_codestream::write_component_packet_header+0xb90>
   e4bb9:	48 c7 44 24 10 00 00 00 00                      	movq   $0x0,0x10(%rsp)
   e4bc2:	49 89 c4                                        	mov    %rax,%r12
   e4bc5:	4c 3b 6c 24 60                                  	cmp    0x60(%rsp),%r13
   e4bca:	0f 85 f6 fb ff ff                               	jne    e47c6 <emuella_j2k_codestream::write_component_packet_header+0x7e6>
   e4bd0:	e9 e6 fb ff ff                                  	jmp    e47bb <emuella_j2k_codestream::write_component_packet_header+0x7db>
   e4bd5:	48 89 7c 24 10                                  	mov    %rdi,0x10(%rsp)
   e4bda:	4c 8b 7c 24 30                                  	mov    0x30(%rsp),%r15
   e4bdf:	48 8b 44 24 38                                  	mov    0x38(%rsp),%rax
   e4be4:	4c 8b ac 24 10 01 00 00                         	mov    0x110(%rsp),%r13
   e4bec:	49 89 c4                                        	mov    %rax,%r12
   e4bef:	4c 3b 6c 24 60                                  	cmp    0x60(%rsp),%r13
   e4bf4:	0f 84 c1 fb ff ff                               	je     e47bb <emuella_j2k_codestream::write_component_packet_header+0x7db>
   e4bfa:	e9 c7 fb ff ff                                  	jmp    e47c6 <emuella_j2k_codestream::write_component_packet_header+0x7e6>
   e4bff:	48 83 bc 24 90 00 00 00 00                      	cmpq   $0x0,0x90(%rsp)
   e4c08:	4c 8b 6c 24 20                                  	mov    0x20(%rsp),%r13
   e4c0d:	0f 84 57 02 00 00                               	je     e4e6a <emuella_j2k_codestream::write_component_packet_header+0xe8a>
   e4c13:	48 8b bc 24 98 00 00 00                         	mov    0x98(%rsp),%rdi
   e4c1b:	ff 15 af 91 18 00                               	call   *0x1891af(%rip)        # 26ddd0 <free@GLIBC_2.2.5>
   e4c21:	e9 44 02 00 00                                  	jmp    e4e6a <emuella_j2k_codestream::write_component_packet_header+0xe8a>
   e4c26:	48 8d 15 fb 25 18 00                            	lea    0x1825fb(%rip),%rdx        # 267228 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x280>
   e4c2d:	ff 15 75 91 18 00                               	call   *0x189175(%rip)        # 26dda8 <_DYNAMIC+0x228>
   e4c33:	e9 f3 04 00 00                                  	jmp    e512b <emuella_j2k_codestream::write_component_packet_header+0x114b>
   e4c38:	48 8b 44 24 70                                  	mov    0x70(%rsp),%rax
   e4c3d:	0f 10 44 24 60                                  	movups 0x60(%rsp),%xmm0
   e4c42:	0f 29 84 24 80 01 00 00                         	movaps %xmm0,0x180(%rsp)
   e4c4a:	0f 29 84 24 40 01 00 00                         	movaps %xmm0,0x140(%rsp)
   e4c52:	48 89 44 24 10                                  	mov    %rax,0x10(%rsp)
   e4c57:	48 89 84 24 50 01 00 00                         	mov    %rax,0x150(%rsp)
   e4c5f:	48 8b 84 24 48 01 00 00                         	mov    0x148(%rsp),%rax
   e4c67:	48 89 84 24 80 00 00 00                         	mov    %rax,0x80(%rsp)
   e4c6f:	48 8b 84 24 a8 00 00 00                         	mov    0xa8(%rsp),%rax
   e4c77:	48 3b 84 24 58 01 00 00                         	cmp    0x158(%rsp),%rax
   e4c7f:	0f 84 7b 02 00 00                               	je     e4f00 <emuella_j2k_codestream::write_component_packet_header+0xf20>
   e4c85:	44 0f b6 60 1b                                  	movzbl 0x1b(%rax),%r12d
   e4c8a:	44 0f b7 78 10                                  	movzwl 0x10(%rax),%r15d
   e4c8f:	49 89 c5                                        	mov    %rax,%r13
   e4c92:	0f b7 68 12                                     	movzwl 0x12(%rax),%ebp
   e4c96:	c7 04 24 01 00 00 00                            	movl   $0x1,(%rsp)
   e4c9d:	48 8d 7c 24 30                                  	lea    0x30(%rsp),%rdi
   e4ca2:	48 89 de                                        	mov    %rbx,%rsi
   e4ca5:	4c 89 f2                                        	mov    %r14,%rdx
   e4ca8:	48 8b 8c 24 b0 00 00 00                         	mov    0xb0(%rsp),%rcx
   e4cb0:	45 89 f8                                        	mov    %r15d,%r8d
   e4cb3:	41 89 e9                                        	mov    %ebp,%r9d
   e4cb6:	e8 45 65 06 00                                  	call   14b200 <<emuella_j2k_codestream::EncTagTree>::encode>
   e4cbb:	48 83 7c 24 30 ff                               	cmpq   $0xffffffffffffffff,0x30(%rsp)
   e4cc1:	0f 85 15 02 00 00                               	jne    e4edc <emuella_j2k_codestream::write_component_packet_header+0xefc>
   e4cc7:	49 8d 45 20                                     	lea    0x20(%r13),%rax
   e4ccb:	45 84 e4                                        	test   %r12b,%r12b
   e4cce:	74 a7                                           	je     e4c77 <emuella_j2k_codestream::write_component_packet_header+0xc97>
   e4cd0:	c7 04 24 ff ff ff ff                            	movl   $0xffffffff,(%rsp)
   e4cd7:	48 8d 7c 24 30                                  	lea    0x30(%rsp),%rdi
   e4cdc:	48 8b b4 24 80 00 00 00                         	mov    0x80(%rsp),%rsi
   e4ce4:	48 8b 54 24 10                                  	mov    0x10(%rsp),%rdx
   e4ce9:	48 8b 8c 24 b0 00 00 00                         	mov    0xb0(%rsp),%rcx
   e4cf1:	45 89 f8                                        	mov    %r15d,%r8d
   e4cf4:	41 89 e9                                        	mov    %ebp,%r9d
   e4cf7:	e8 04 65 06 00                                  	call   14b200 <<emuella_j2k_codestream::EncTagTree>::encode>
   e4cfc:	48 83 7c 24 30 ff                               	cmpq   $0xffffffffffffffff,0x30(%rsp)
   e4d02:	0f 85 d4 01 00 00                               	jne    e4edc <emuella_j2k_codestream::write_component_packet_header+0xefc>
   e4d08:	49 83 c5 20                                     	add    $0x20,%r13
   e4d0c:	4c 89 ac 24 a8 00 00 00                         	mov    %r13,0xa8(%rsp)
   e4d14:	41 0f b7 6d f8                                  	movzwl -0x8(%r13),%ebp
   e4d19:	48 8d 7c 24 30                                  	lea    0x30(%rsp),%rdi
   e4d1e:	48 8b b4 24 b0 00 00 00                         	mov    0xb0(%rsp),%rsi
   e4d26:	89 ea                                           	mov    %ebp,%edx
   e4d28:	e8 63 0a ff ff                                  	call   d5790 <emuella_j2k_codestream::write_coding_pass_count>
   e4d2d:	48 83 7c 24 30 ff                               	cmpq   $0xffffffffffffffff,0x30(%rsp)
   e4d33:	0f 85 a3 01 00 00                               	jne    e4edc <emuella_j2k_codestream::write_component_packet_header+0xefc>
   e4d39:	85 ed                                           	test   %ebp,%ebp
   e4d3b:	4c 8b ac 24 b0 00 00 00                         	mov    0xb0(%rsp),%r13
   e4d43:	0f 84 89 03 00 00                               	je     e50d2 <emuella_j2k_codestream::write_component_packet_header+0x10f2>
   e4d49:	48 8b 84 24 a8 00 00 00                         	mov    0xa8(%rsp),%rax
   e4d51:	4c 8b 60 e8                                     	mov    -0x18(%rax),%r12
   e4d55:	0f bd c5                                        	bsr    %ebp,%eax
   e4d58:	48 89 44 24 18                                  	mov    %rax,0x18(%rsp)
   e4d5d:	83 f0 1f                                        	xor    $0x1f,%eax
   e4d60:	41 b7 22                                        	mov    $0x22,%r15b
   e4d63:	41 28 c7                                        	sub    %al,%r15b
   e4d66:	4c 89 e0                                        	mov    %r12,%rax
   e4d69:	44 89 f9                                        	mov    %r15d,%ecx
   e4d6c:	48 d3 e8                                        	shr    %cl,%rax
   e4d6f:	48 85 c0                                        	test   %rax,%rax
   e4d72:	74 61                                           	je     e4dd5 <emuella_j2k_codestream::write_component_packet_header+0xdf5>
   e4d74:	40 b5 fd                                        	mov    $0xfd,%bpl
   e4d77:	41 b7 04                                        	mov    $0x4,%r15b
   e4d7a:	48 8d 7c 24 30                                  	lea    0x30(%rsp),%rdi
   e4d7f:	4c 89 ee                                        	mov    %r13,%rsi
   e4d82:	ba 01 00 00 00                                  	mov    $0x1,%edx
   e4d87:	e8 c4 6d 06 00                                  	call   14bb50 <<emuella_j2k_codestream::PacketBitWriter>::write_bit>
   e4d8c:	48 8b 44 24 30                                  	mov    0x30(%rsp),%rax
   e4d91:	48 83 f8 ff                                     	cmp    $0xffffffffffffffff,%rax
   e4d95:	0f 85 0e 02 00 00                               	jne    e4fa9 <emuella_j2k_codestream::write_component_packet_header+0xfc9>
   e4d9b:	45 84 ff                                        	test   %r15b,%r15b
   e4d9e:	0f 84 18 03 00 00                               	je     e50bc <emuella_j2k_codestream::write_component_packet_header+0x10dc>
   e4da4:	48 8b 44 24 18                                  	mov    0x18(%rsp),%rax
   e4da9:	42 8d 0c 38                                     	lea    (%rax,%r15,1),%ecx
   e4dad:	4c 89 e0                                        	mov    %r12,%rax
   e4db0:	31 d2                                           	xor    %edx,%edx
   e4db2:	48 0f ad d0                                     	shrd   %cl,%rdx,%rax
   e4db6:	f6 c1 40                                        	test   $0x40,%cl
   e4db9:	48 0f 45 c2                                     	cmovne %rdx,%rax
   e4dbd:	40 fe cd                                        	dec    %bpl
   e4dc0:	41 fe c7                                        	inc    %r15b
   e4dc3:	48 85 c0                                        	test   %rax,%rax
   e4dc6:	75 b2                                           	jne    e4d7a <emuella_j2k_codestream::write_component_packet_header+0xd9a>
   e4dc8:	4c 8b 7c 24 18                                  	mov    0x18(%rsp),%r15
   e4dcd:	41 28 ef                                        	sub    %bpl,%r15b
   e4dd0:	40 f6 dd                                        	neg    %bpl
   e4dd3:	eb 03                                           	jmp    e4dd8 <emuella_j2k_codestream::write_component_packet_header+0xdf8>
   e4dd5:	40 b5 03                                        	mov    $0x3,%bpl
   e4dd8:	48 8d 7c 24 30                                  	lea    0x30(%rsp),%rdi
   e4ddd:	4c 89 ee                                        	mov    %r13,%rsi
   e4de0:	31 d2                                           	xor    %edx,%edx
   e4de2:	e8 69 6d 06 00                                  	call   14bb50 <<emuella_j2k_codestream::PacketBitWriter>::write_bit>
   e4de7:	48 8b 44 24 30                                  	mov    0x30(%rsp),%rax
   e4dec:	48 83 f8 ff                                     	cmp    $0xffffffffffffffff,%rax
   e4df0:	0f 85 b3 01 00 00                               	jne    e4fa9 <emuella_j2k_codestream::write_component_packet_header+0xfc9>
   e4df6:	4d 89 fd                                        	mov    %r15,%r13
   e4df9:	41 38 ef                                        	cmp    %bpl,%r15b
   e4dfc:	4c 8b bc 24 b0 00 00 00                         	mov    0xb0(%rsp),%r15
   e4e04:	0f 82 e7 02 00 00                               	jb     e50f1 <emuella_j2k_codestream::write_component_packet_header+0x1111>
   e4e0a:	4c 89 e0                                        	mov    %r12,%rax
   e4e0d:	48 c1 e8 20                                     	shr    $0x20,%rax
   e4e11:	0f 85 94 02 00 00                               	jne    e50ab <emuella_j2k_codestream::write_component_packet_header+0x10cb>
   e4e17:	41 8d 6d ff                                     	lea    -0x1(%r13),%ebp
   e4e1b:	40 0f b6 c5                                     	movzbl %bpl,%eax
   e4e1f:	31 d2                                           	xor    %edx,%edx
   e4e21:	41 0f a3 c4                                     	bt     %eax,%r12d
   e4e25:	0f 92 c2                                        	setb   %dl
   e4e28:	48 8d 7c 24 30                                  	lea    0x30(%rsp),%rdi
   e4e2d:	4c 89 fe                                        	mov    %r15,%rsi
   e4e30:	e8 1b 6d 06 00                                  	call   14bb50 <<emuella_j2k_codestream::PacketBitWriter>::write_bit>
   e4e35:	48 8b 44 24 30                                  	mov    0x30(%rsp),%rax
   e4e3a:	48 83 f8 ff                                     	cmp    $0xffffffffffffffff,%rax
   e4e3e:	0f 85 13 02 00 00                               	jne    e5057 <emuella_j2k_codestream::write_component_packet_header+0x1077>
   e4e44:	40 80 c5 ff                                     	add    $0xff,%bpl
   e4e48:	72 d1                                           	jb     e4e1b <emuella_j2k_codestream::write_component_packet_header+0xe3b>
   e4e4a:	44 88 6c 24 28                                  	mov    %r13b,0x28(%rsp)
   e4e4f:	e9 1b fe ff ff                                  	jmp    e4c6f <emuella_j2k_codestream::write_component_packet_header+0xc8f>
   e4e54:	48 83 7c 24 60 00                               	cmpq   $0x0,0x60(%rsp)
   e4e5a:	74 09                                           	je     e4e65 <emuella_j2k_codestream::write_component_packet_header+0xe85>
   e4e5c:	48 89 ef                                        	mov    %rbp,%rdi
   e4e5f:	ff 15 6b 8f 18 00                               	call   *0x188f6b(%rip)        # 26ddd0 <free@GLIBC_2.2.5>
   e4e65:	4c 8b 6c 24 20                                  	mov    0x20(%rsp),%r13
   e4e6a:	0f 10 84 24 a0 01 00 00                         	movups 0x1a0(%rsp),%xmm0
   e4e72:	0f 29 84 24 80 01 00 00                         	movaps %xmm0,0x180(%rsp)
   e4e7a:	48 8b 84 24 b0 01 00 00                         	mov    0x1b0(%rsp),%rax
   e4e82:	48 89 84 24 90 01 00 00                         	mov    %rax,0x190(%rsp)
   e4e8a:	48 8b 4c 24 78                                  	mov    0x78(%rsp),%rcx
   e4e8f:	48 89 41 18                                     	mov    %rax,0x18(%rcx)
   e4e93:	0f 11 41 08                                     	movups %xmm0,0x8(%rcx)
   e4e97:	48 b8 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rax
   e4ea1:	48 89 01                                        	mov    %rax,(%rcx)
   e4ea4:	4c 8d 7b 08                                     	lea    0x8(%rbx),%r15
   e4ea8:	4c 8b 25 21 8f 18 00                            	mov    0x188f21(%rip),%r12        # 26ddd0 <free@GLIBC_2.2.5>
   e4eaf:	eb 1c                                           	jmp    e4ecd <emuella_j2k_codestream::write_component_packet_header+0xeed>
   e4eb1:	66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00    	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   e4ec0:	49 83 c7 20                                     	add    $0x20,%r15
   e4ec4:	49 ff ce                                        	dec    %r14
   e4ec7:	0f 84 b9 00 00 00                               	je     e4f86 <emuella_j2k_codestream::write_component_packet_header+0xfa6>
   e4ecd:	49 83 7f f8 00                                  	cmpq   $0x0,-0x8(%r15)
   e4ed2:	74 ec                                           	je     e4ec0 <emuella_j2k_codestream::write_component_packet_header+0xee0>
   e4ed4:	49 8b 3f                                        	mov    (%r15),%rdi
   e4ed7:	41 ff d4                                        	call   *%r12
   e4eda:	eb e4                                           	jmp    e4ec0 <emuella_j2k_codestream::write_component_packet_header+0xee0>
   e4edc:	0f 10 44 24 30                                  	movups 0x30(%rsp),%xmm0
   e4ee1:	0f 10 4c 24 40                                  	movups 0x40(%rsp),%xmm1
   e4ee6:	0f 10 54 24 50                                  	movups 0x50(%rsp),%xmm2
   e4eeb:	48 8b 44 24 78                                  	mov    0x78(%rsp),%rax
   e4ef0:	0f 11 50 20                                     	movups %xmm2,0x20(%rax)
   e4ef4:	0f 11 48 10                                     	movups %xmm1,0x10(%rax)
   e4ef8:	0f 11 00                                        	movups %xmm0,(%rax)
   e4efb:	e9 1b 01 00 00                                  	jmp    e501b <emuella_j2k_codestream::write_component_packet_header+0x103b>
   e4f00:	48 8b 44 24 78                                  	mov    0x78(%rsp),%rax
   e4f05:	48 c7 00 ff ff ff ff                            	movq   $0xffffffffffffffff,(%rax)
   e4f0c:	48 8b 6c 24 10                                  	mov    0x10(%rsp),%rbp
   e4f11:	48 85 ed                                        	test   %rbp,%rbp
   e4f14:	4c 8b 6c 24 20                                  	mov    0x20(%rsp),%r13
   e4f19:	74 2d                                           	je     e4f48 <emuella_j2k_codestream::write_component_packet_header+0xf68>
   e4f1b:	48 8b 84 24 80 00 00 00                         	mov    0x80(%rsp),%rax
   e4f23:	4c 8d 78 08                                     	lea    0x8(%rax),%r15
   e4f27:	4c 8b 25 a2 8e 18 00                            	mov    0x188ea2(%rip),%r12        # 26ddd0 <free@GLIBC_2.2.5>
   e4f2e:	eb 09                                           	jmp    e4f39 <emuella_j2k_codestream::write_component_packet_header+0xf59>
   e4f30:	49 83 c7 20                                     	add    $0x20,%r15
   e4f34:	48 ff cd                                        	dec    %rbp
   e4f37:	74 0f                                           	je     e4f48 <emuella_j2k_codestream::write_component_packet_header+0xf68>
   e4f39:	49 83 7f f8 00                                  	cmpq   $0x0,-0x8(%r15)
   e4f3e:	74 f0                                           	je     e4f30 <emuella_j2k_codestream::write_component_packet_header+0xf50>
   e4f40:	49 8b 3f                                        	mov    (%r15),%rdi
   e4f43:	41 ff d4                                        	call   *%r12
   e4f46:	eb e8                                           	jmp    e4f30 <emuella_j2k_codestream::write_component_packet_header+0xf50>
   e4f48:	48 83 bc 24 40 01 00 00 00                      	cmpq   $0x0,0x140(%rsp)
   e4f51:	74 0e                                           	je     e4f61 <emuella_j2k_codestream::write_component_packet_header+0xf81>
   e4f53:	48 8b bc 24 80 00 00 00                         	mov    0x80(%rsp),%rdi
   e4f5b:	ff 15 6f 8e 18 00                               	call   *0x188e6f(%rip)        # 26ddd0 <free@GLIBC_2.2.5>
   e4f61:	4c 8d 7b 08                                     	lea    0x8(%rbx),%r15
   e4f65:	4c 8b 25 64 8e 18 00                            	mov    0x188e64(%rip),%r12        # 26ddd0 <free@GLIBC_2.2.5>
   e4f6c:	eb 09                                           	jmp    e4f77 <emuella_j2k_codestream::write_component_packet_header+0xf97>
   e4f6e:	49 83 c7 20                                     	add    $0x20,%r15
   e4f72:	49 ff ce                                        	dec    %r14
   e4f75:	74 0f                                           	je     e4f86 <emuella_j2k_codestream::write_component_packet_header+0xfa6>
   e4f77:	49 83 7f f8 00                                  	cmpq   $0x0,-0x8(%r15)
   e4f7c:	74 f0                                           	je     e4f6e <emuella_j2k_codestream::write_component_packet_header+0xf8e>
   e4f7e:	49 8b 3f                                        	mov    (%r15),%rdi
   e4f81:	41 ff d4                                        	call   *%r12
   e4f84:	eb e8                                           	jmp    e4f6e <emuella_j2k_codestream::write_component_packet_header+0xf8e>
   e4f86:	4d 85 ed                                        	test   %r13,%r13
   e4f89:	0f 84 9d f6 ff ff                               	je     e462c <emuella_j2k_codestream::write_component_packet_header+0x64c>
   e4f8f:	48 89 df                                        	mov    %rbx,%rdi
   e4f92:	48 81 c4 b8 01 00 00                            	add    $0x1b8,%rsp
   e4f99:	5b                                              	pop    %rbx
   e4f9a:	41 5c                                           	pop    %r12
   e4f9c:	41 5d                                           	pop    %r13
   e4f9e:	41 5e                                           	pop    %r14
   e4fa0:	41 5f                                           	pop    %r15
   e4fa2:	5d                                              	pop    %rbp
   e4fa3:	ff 25 27 8e 18 00                               	jmp    *0x188e27(%rip)        # 26ddd0 <free@GLIBC_2.2.5>
   e4fa9:	0f b6 74 24 38                                  	movzbl 0x38(%rsp),%esi
   e4fae:	0f b6 4c 24 3f                                  	movzbl 0x3f(%rsp),%ecx
   e4fb3:	c1 e1 10                                        	shl    $0x10,%ecx
   e4fb6:	0f b7 54 24 3d                                  	movzwl 0x3d(%rsp),%edx
   e4fbb:	09 ca                                           	or     %ecx,%edx
   e4fbd:	48 c1 e2 20                                     	shl    $0x20,%rdx
   e4fc1:	8b 4c 24 39                                     	mov    0x39(%rsp),%ecx
   e4fc5:	48 09 d1                                        	or     %rdx,%rcx
   e4fc8:	0f 10 44 24 40                                  	movups 0x40(%rsp),%xmm0
   e4fcd:	0f 29 84 24 60 01 00 00                         	movaps %xmm0,0x160(%rsp)
   e4fd5:	0f 10 44 24 50                                  	movups 0x50(%rsp),%xmm0
   e4fda:	0f 29 84 24 70 01 00 00                         	movaps %xmm0,0x170(%rsp)
   e4fe2:	48 8b 54 24 78                                  	mov    0x78(%rsp),%rdx
   e4fe7:	48 89 02                                        	mov    %rax,(%rdx)
   e4fea:	40 88 72 08                                     	mov    %sil,0x8(%rdx)
   e4fee:	89 4a 09                                        	mov    %ecx,0x9(%rdx)
   e4ff1:	48 89 c8                                        	mov    %rcx,%rax
   e4ff4:	48 c1 e8 30                                     	shr    $0x30,%rax
   e4ff8:	88 42 0f                                        	mov    %al,0xf(%rdx)
   e4ffb:	48 c1 e9 20                                     	shr    $0x20,%rcx
   e4fff:	66 89 4a 0d                                     	mov    %cx,0xd(%rdx)
   e5003:	0f 28 84 24 60 01 00 00                         	movaps 0x160(%rsp),%xmm0
   e500b:	0f 28 8c 24 70 01 00 00                         	movaps 0x170(%rsp),%xmm1
   e5013:	0f 11 42 10                                     	movups %xmm0,0x10(%rdx)
   e5017:	0f 11 4a 20                                     	movups %xmm1,0x20(%rdx)
   e501b:	4c 8b 6c 24 20                                  	mov    0x20(%rsp),%r13
   e5020:	48 8b 6c 24 10                                  	mov    0x10(%rsp),%rbp
   e5025:	48 85 ed                                        	test   %rbp,%rbp
   e5028:	74 5f                                           	je     e5089 <emuella_j2k_codestream::write_component_packet_header+0x10a9>
   e502a:	48 8b 84 24 80 00 00 00                         	mov    0x80(%rsp),%rax
   e5032:	4c 8d 78 08                                     	lea    0x8(%rax),%r15
   e5036:	4c 8b 25 93 8d 18 00                            	mov    0x188d93(%rip),%r12        # 26ddd0 <free@GLIBC_2.2.5>
   e503d:	eb 09                                           	jmp    e5048 <emuella_j2k_codestream::write_component_packet_header+0x1068>
   e503f:	49 83 c7 20                                     	add    $0x20,%r15
   e5043:	48 ff cd                                        	dec    %rbp
   e5046:	74 41                                           	je     e5089 <emuella_j2k_codestream::write_component_packet_header+0x10a9>
   e5048:	49 83 7f f8 00                                  	cmpq   $0x0,-0x8(%r15)
   e504d:	74 f0                                           	je     e503f <emuella_j2k_codestream::write_component_packet_header+0x105f>
   e504f:	49 8b 3f                                        	mov    (%r15),%rdi
   e5052:	41 ff d4                                        	call   *%r12
   e5055:	eb e8                                           	jmp    e503f <emuella_j2k_codestream::write_component_packet_header+0x105f>
   e5057:	48 8b 4c 24 58                                  	mov    0x58(%rsp),%rcx
   e505c:	48 8b 54 24 78                                  	mov    0x78(%rsp),%rdx
   e5061:	48 89 4a 28                                     	mov    %rcx,0x28(%rdx)
   e5065:	0f 10 44 24 38                                  	movups 0x38(%rsp),%xmm0
   e506a:	0f 10 4c 24 48                                  	movups 0x48(%rsp),%xmm1
   e506f:	0f 11 4a 18                                     	movups %xmm1,0x18(%rdx)
   e5073:	0f 11 42 08                                     	movups %xmm0,0x8(%rdx)
   e5077:	48 8b 6c 24 10                                  	mov    0x10(%rsp),%rbp
   e507c:	48 89 02                                        	mov    %rax,(%rdx)
   e507f:	4c 8b 6c 24 20                                  	mov    0x20(%rsp),%r13
   e5084:	48 85 ed                                        	test   %rbp,%rbp
   e5087:	75 a1                                           	jne    e502a <emuella_j2k_codestream::write_component_packet_header+0x104a>
   e5089:	48 83 bc 24 40 01 00 00 00                      	cmpq   $0x0,0x140(%rsp)
   e5092:	0f 84 0c fe ff ff                               	je     e4ea4 <emuella_j2k_codestream::write_component_packet_header+0xec4>
   e5098:	48 8b bc 24 80 00 00 00                         	mov    0x80(%rsp),%rdi
   e50a0:	ff 15 2a 8d 18 00                               	call   *0x188d2a(%rip)        # 26ddd0 <free@GLIBC_2.2.5>
   e50a6:	e9 f9 fd ff ff                                  	jmp    e4ea4 <emuella_j2k_codestream::write_component_packet_header+0xec4>
   e50ab:	48 b8 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rax
   e50b5:	48 8b 54 24 78                                  	mov    0x78(%rsp),%rdx
   e50ba:	eb bb                                           	jmp    e5077 <emuella_j2k_codestream::write_component_packet_header+0x1097>
   e50bc:	48 b8 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rax
   e50c6:	31 c9                                           	xor    %ecx,%ecx
   e50c8:	0f b6 74 24 28                                  	movzbl 0x28(%rsp),%esi
   e50cd:	e9 10 ff ff ff                                  	jmp    e4fe2 <emuella_j2k_codestream::write_component_packet_header+0x1002>
   e50d2:	48 8d 3d e7 2b 18 00                            	lea    0x182be7(%rip),%rdi        # 267cc0 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0xd18>
   e50d9:	ff 15 51 90 18 00                               	call   *0x189051(%rip)        # 26e130 <_DYNAMIC+0x5b0>
   e50df:	eb 4a                                           	jmp    e512b <emuella_j2k_codestream::write_component_packet_header+0x114b>
   e50e1:	bf 04 00 00 00                                  	mov    $0x4,%edi
   e50e6:	4c 89 ee                                        	mov    %r13,%rsi
   e50e9:	ff 15 a9 8c 18 00                               	call   *0x188ca9(%rip)        # 26dd98 <_DYNAMIC+0x218>
   e50ef:	eb 3a                                           	jmp    e512b <emuella_j2k_codestream::write_component_packet_header+0x114b>
   e50f1:	48 b8 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rax
   e50fb:	eb cb                                           	jmp    e50c8 <emuella_j2k_codestream::write_component_packet_header+0x10e8>
   e50fd:	bf 04 00 00 00                                  	mov    $0x4,%edi
   e5102:	48 8b 74 24 20                                  	mov    0x20(%rsp),%rsi
   e5107:	ff 15 8b 8c 18 00                               	call   *0x188c8b(%rip)        # 26dd98 <_DYNAMIC+0x218>
   e510d:	bf 04 00 00 00                                  	mov    $0x4,%edi
   e5112:	4c 89 e6                                        	mov    %r12,%rsi
   e5115:	ff 15 7d 8c 18 00                               	call   *0x188c7d(%rip)        # 26dd98 <_DYNAMIC+0x218>
   e511b:	eb 0e                                           	jmp    e512b <emuella_j2k_codestream::write_component_packet_header+0x114b>
   e511d:	bf 04 00 00 00                                  	mov    $0x4,%edi
   e5122:	4c 89 fe                                        	mov    %r15,%rsi
   e5125:	ff 15 6d 8c 18 00                               	call   *0x188c6d(%rip)        # 26dd98 <_DYNAMIC+0x218>
   e512b:	0f 0b                                           	ud2
   e512d:	e9 a2 00 00 00                                  	jmp    e51d4 <emuella_j2k_codestream::write_component_packet_header+0x11f4>
   e5132:	e9 9d 00 00 00                                  	jmp    e51d4 <emuella_j2k_codestream::write_component_packet_header+0x11f4>
   e5137:	e9 98 00 00 00                                  	jmp    e51d4 <emuella_j2k_codestream::write_component_packet_header+0x11f4>
   e513c:	49 89 c4                                        	mov    %rax,%r12
   e513f:	4d 85 ed                                        	test   %r13,%r13
   e5142:	0f 84 e1 00 00 00                               	je     e5229 <emuella_j2k_codestream::write_component_packet_header+0x1249>
   e5148:	4c 89 ff                                        	mov    %r15,%rdi
   e514b:	e9 d3 00 00 00                                  	jmp    e5223 <emuella_j2k_codestream::write_component_packet_header+0x1243>
   e5150:	49 89 c4                                        	mov    %rax,%r12
   e5153:	e9 fd 00 00 00                                  	jmp    e5255 <emuella_j2k_codestream::write_component_packet_header+0x1275>
   e5158:	eb 7a                                           	jmp    e51d4 <emuella_j2k_codestream::write_component_packet_header+0x11f4>
   e515a:	49 89 c5                                        	mov    %rax,%r13
   e515d:	4d 85 ff                                        	test   %r15,%r15
   e5160:	75 08                                           	jne    e516a <emuella_j2k_codestream::write_component_packet_header+0x118a>
   e5162:	4d 89 ec                                        	mov    %r13,%r12
   e5165:	e9 bf 00 00 00                                  	jmp    e5229 <emuella_j2k_codestream::write_component_packet_header+0x1249>
   e516a:	4c 89 e7                                        	mov    %r12,%rdi
   e516d:	ff 15 5d 8c 18 00                               	call   *0x188c5d(%rip)        # 26ddd0 <free@GLIBC_2.2.5>
   e5173:	4d 89 ec                                        	mov    %r13,%r12
   e5176:	e9 ae 00 00 00                                  	jmp    e5229 <emuella_j2k_codestream::write_component_packet_header+0x1249>
   e517b:	49 89 c4                                        	mov    %rax,%r12
   e517e:	e9 a6 00 00 00                                  	jmp    e5229 <emuella_j2k_codestream::write_component_packet_header+0x1249>
   e5183:	49 89 c4                                        	mov    %rax,%r12
   e5186:	4d 85 ff                                        	test   %r15,%r15
   e5189:	0f 84 50 01 00 00                               	je     e52df <emuella_j2k_codestream::write_component_packet_header+0x12ff>
   e518f:	4c 89 f7                                        	mov    %r14,%rdi
   e5192:	e9 42 01 00 00                                  	jmp    e52d9 <emuella_j2k_codestream::write_component_packet_header+0x12f9>
   e5197:	49 89 c4                                        	mov    %rax,%r12
   e519a:	48 83 bc 24 90 00 00 00 00                      	cmpq   $0x0,0x90(%rsp)
   e51a3:	0f 84 a7 00 00 00                               	je     e5250 <emuella_j2k_codestream::write_component_packet_header+0x1270>
   e51a9:	48 8b bc 24 98 00 00 00                         	mov    0x98(%rsp),%rdi
   e51b1:	e9 94 00 00 00                                  	jmp    e524a <emuella_j2k_codestream::write_component_packet_header+0x126a>
   e51b6:	49 89 c4                                        	mov    %rax,%r12
   e51b9:	4d 85 ff                                        	test   %r15,%r15
   e51bc:	0f 84 1d 01 00 00                               	je     e52df <emuella_j2k_codestream::write_component_packet_header+0x12ff>
   e51c2:	48 8b 7c 24 18                                  	mov    0x18(%rsp),%rdi
   e51c7:	e9 0d 01 00 00                                  	jmp    e52d9 <emuella_j2k_codestream::write_component_packet_header+0x12f9>
   e51cc:	49 89 c4                                        	mov    %rax,%r12
   e51cf:	e9 0b 01 00 00                                  	jmp    e52df <emuella_j2k_codestream::write_component_packet_header+0x12ff>
   e51d4:	49 89 c4                                        	mov    %rax,%r12
   e51d7:	48 8d bc 24 40 01 00 00                         	lea    0x140(%rsp),%rdi
   e51df:	e8 8c 2d fb ff                                  	call   97f70 <core::ptr::drop_glue::<emuella_j2k_codestream::EncTagTree>>
   e51e4:	eb 6a                                           	jmp    e5250 <emuella_j2k_codestream::write_component_packet_header+0x1270>
   e51e6:	49 89 c4                                        	mov    %rax,%r12
   e51e9:	48 83 bc 24 90 00 00 00 00                      	cmpq   $0x0,0x90(%rsp)
   e51f2:	0f 84 07 01 00 00                               	je     e52ff <emuella_j2k_codestream::write_component_packet_header+0x131f>
   e51f8:	48 8b 9c 24 98 00 00 00                         	mov    0x98(%rsp),%rbx
   e5200:	e9 f1 00 00 00                                  	jmp    e52f6 <emuella_j2k_codestream::write_component_packet_header+0x1316>
   e5205:	eb 0c                                           	jmp    e5213 <emuella_j2k_codestream::write_component_packet_header+0x1233>
   e5207:	eb 0a                                           	jmp    e5213 <emuella_j2k_codestream::write_component_packet_header+0x1233>
   e5209:	e9 bb 00 00 00                                  	jmp    e52c9 <emuella_j2k_codestream::write_component_packet_header+0x12e9>
   e520e:	e9 b6 00 00 00                                  	jmp    e52c9 <emuella_j2k_codestream::write_component_packet_header+0x12e9>
   e5213:	49 89 c4                                        	mov    %rax,%r12
   e5216:	48 83 7c 24 30 00                               	cmpq   $0x0,0x30(%rsp)
   e521c:	74 0b                                           	je     e5229 <emuella_j2k_codestream::write_component_packet_header+0x1249>
   e521e:	48 8b 7c 24 38                                  	mov    0x38(%rsp),%rdi
   e5223:	ff 15 a7 8b 18 00                               	call   *0x188ba7(%rip)        # 26ddd0 <free@GLIBC_2.2.5>
   e5229:	48 8b 44 24 68                                  	mov    0x68(%rsp),%rax
   e522e:	48 89 44 24 18                                  	mov    %rax,0x18(%rsp)
   e5233:	4c 8b 6c 24 70                                  	mov    0x70(%rsp),%r13
   e5238:	4d 85 ed                                        	test   %r13,%r13
   e523b:	75 52                                           	jne    e528f <emuella_j2k_codestream::write_component_packet_header+0x12af>
   e523d:	48 83 7c 24 60 00                               	cmpq   $0x0,0x60(%rsp)
   e5243:	74 0b                                           	je     e5250 <emuella_j2k_codestream::write_component_packet_header+0x1270>
   e5245:	48 8b 7c 24 18                                  	mov    0x18(%rsp),%rdi
   e524a:	ff 15 80 8b 18 00                               	call   *0x188b80(%rip)        # 26ddd0 <free@GLIBC_2.2.5>
   e5250:	48 8b 6c 24 20                                  	mov    0x20(%rsp),%rbp
   e5255:	4c 8d 7b 08                                     	lea    0x8(%rbx),%r15
   e5259:	4c 8b 2d 70 8b 18 00                            	mov    0x188b70(%rip),%r13        # 26ddd0 <free@GLIBC_2.2.5>
   e5260:	eb 17                                           	jmp    e5279 <emuella_j2k_codestream::write_component_packet_header+0x1299>
   e5262:	66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00       	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   e5270:	49 83 c7 20                                     	add    $0x20,%r15
   e5274:	49 ff ce                                        	dec    %r14
   e5277:	74 0f                                           	je     e5288 <emuella_j2k_codestream::write_component_packet_header+0x12a8>
   e5279:	49 83 7f f8 00                                  	cmpq   $0x0,-0x8(%r15)
   e527e:	74 f0                                           	je     e5270 <emuella_j2k_codestream::write_component_packet_header+0x1290>
   e5280:	49 8b 3f                                        	mov    (%r15),%rdi
   e5283:	41 ff d5                                        	call   *%r13
   e5286:	eb e8                                           	jmp    e5270 <emuella_j2k_codestream::write_component_packet_header+0x1290>
   e5288:	48 85 ed                                        	test   %rbp,%rbp
   e528b:	75 69                                           	jne    e52f6 <emuella_j2k_codestream::write_component_packet_header+0x1316>
   e528d:	eb 70                                           	jmp    e52ff <emuella_j2k_codestream::write_component_packet_header+0x131f>
   e528f:	48 8b 44 24 18                                  	mov    0x18(%rsp),%rax
   e5294:	48 8d 68 08                                     	lea    0x8(%rax),%rbp
   e5298:	4c 8b 3d 31 8b 18 00                            	mov    0x188b31(%rip),%r15        # 26ddd0 <free@GLIBC_2.2.5>
   e529f:	eb 18                                           	jmp    e52b9 <emuella_j2k_codestream::write_component_packet_header+0x12d9>
   e52a1:	66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00    	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   e52b0:	48 83 c5 20                                     	add    $0x20,%rbp
   e52b4:	49 ff cd                                        	dec    %r13
   e52b7:	74 84                                           	je     e523d <emuella_j2k_codestream::write_component_packet_header+0x125d>
   e52b9:	48 83 7d f8 00                                  	cmpq   $0x0,-0x8(%rbp)
   e52be:	74 f0                                           	je     e52b0 <emuella_j2k_codestream::write_component_packet_header+0x12d0>
   e52c0:	48 8b 7d 00                                     	mov    0x0(%rbp),%rdi
   e52c4:	41 ff d7                                        	call   *%r15
   e52c7:	eb e7                                           	jmp    e52b0 <emuella_j2k_codestream::write_component_packet_header+0x12d0>
   e52c9:	49 89 c4                                        	mov    %rax,%r12
   e52cc:	48 83 7c 24 30 00                               	cmpq   $0x0,0x30(%rsp)
   e52d2:	74 0b                                           	je     e52df <emuella_j2k_codestream::write_component_packet_header+0x12ff>
   e52d4:	48 8b 7c 24 38                                  	mov    0x38(%rsp),%rdi
   e52d9:	ff 15 f1 8a 18 00                               	call   *0x188af1(%rip)        # 26ddd0 <free@GLIBC_2.2.5>
   e52df:	48 8b 5c 24 68                                  	mov    0x68(%rsp),%rbx
   e52e4:	4c 8b 74 24 70                                  	mov    0x70(%rsp),%r14
   e52e9:	4d 85 f6                                        	test   %r14,%r14
   e52ec:	75 19                                           	jne    e5307 <emuella_j2k_codestream::write_component_packet_header+0x1327>
   e52ee:	48 83 7c 24 60 00                               	cmpq   $0x0,0x60(%rsp)
   e52f4:	74 09                                           	je     e52ff <emuella_j2k_codestream::write_component_packet_header+0x131f>
   e52f6:	48 89 df                                        	mov    %rbx,%rdi
   e52f9:	ff 15 d1 8a 18 00                               	call   *0x188ad1(%rip)        # 26ddd0 <free@GLIBC_2.2.5>
   e52ff:	4c 89 e7                                        	mov    %r12,%rdi
   e5302:	e8 a9 01 18 00                                  	call   2654b0 <_Unwind_Resume@plt>
   e5307:	4c 8d 7b 08                                     	lea    0x8(%rbx),%r15
   e530b:	4c 8b 2d be 8a 18 00                            	mov    0x188abe(%rip),%r13        # 26ddd0 <free@GLIBC_2.2.5>
   e5312:	eb 15                                           	jmp    e5329 <emuella_j2k_codestream::write_component_packet_header+0x1349>
   e5314:	66 66 66 2e 0f 1f 84 00 00 00 00 00             	data16 data16 cs nopw 0x0(%rax,%rax,1)
   e5320:	49 83 c7 20                                     	add    $0x20,%r15
   e5324:	49 ff ce                                        	dec    %r14
   e5327:	74 c5                                           	je     e52ee <emuella_j2k_codestream::write_component_packet_header+0x130e>
   e5329:	49 83 7f f8 00                                  	cmpq   $0x0,-0x8(%r15)
   e532e:	74 f0                                           	je     e5320 <emuella_j2k_codestream::write_component_packet_header+0x1340>
   e5330:	49 8b 3f                                        	mov    (%r15),%rdi
   e5333:	41 ff d5                                        	call   *%r13
   e5336:	eb e8                                           	jmp    e5320 <emuella_j2k_codestream::write_component_packet_header+0x1340>
