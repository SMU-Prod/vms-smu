Bem vindo ao Manual do Digifort Professional
Este Manual do Usuário e Referências Técnicas provê toda
informação necessária para efetivamente implementar e
usar todos os recursos básicos e avançados encontrados
no Cliente de Monitoramento.
Este manual está em constante atualização e não descreve
as funcionalidades das versões betas do sistema
1.1 Screen Shots
Os screen shots contidos nesse manual podem não ser idênticos à interface que você irá ver usando o
software. Algumas diferenças podem aparecer, não prejudicando o uso deste manual. Isto se deve ao
fato de que freqüentes atualizações e inclusão de novos recursos são realizadas objetivando o contínuo
melhoramento do sistema.
1.2 A quem se destina este manual
Este manual se destina à administradores e operadores de estações de monitoramento.
1.3 Como utilizar este manual
Este manual está estruturado em capítulos, tópicos e sub-tópicos.
Importante:
· Caso sua edição não seja a Enterprise, alguns recursos apresentados pode apresentar limitações.
Para conhecer as limitações de sua versão consulte a tabela Matriz de Recursos no site
www.digifort.com
· As capturas de telas desse manual são originalmente tiradas da edição Enterprise. Por esse motivo,
mesmo em outras versões algum recurso pode apresentar uma captura com diferença de tela da
versão de seu software. Estamos constantemente atualizando esse manual e melhorando seu
conteúdo.
1.4 Pré-requisitos
Para a completa absorção do conteúdo desse manual alguns pré-requisitos são necessários:
· Manuseio de computadores e seus periféricos.
· Manuseio do sistema operacional Microsoft Windows.
· Conhecimento da arquitetura cliente-servidor.
· Conhecimento da arquitetura de redes de computadores.
C
h
a
p
t
e
r
I
I
14 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
2 O Cliente de Monitoramento
O Cliente de Monitoramento é o módulo responsável pelo monitoramento das câmeras e recebimento de
alertas configurados pelo administrador.
O sistema permite o monitoramento simultâneo de varias câmeras na mesma tela através de mosaicos,
que podem ser criados dinamicamente. Oferece a capacidade de controle de câmeras móveis com a
função PTZ (Pan Tilt Zoom) e controle de IO, permitindo o disparo de alarmes, abertura de portas
eletrônicas e recebimentos de alertas de sensores de movimento.
Assim como o Cliente de Administração, o Cliente de Monitoramento também tem a capacidade de
monitorar vários servidores simultaneamente. Com este recurso câmeras de diversas câmeras podem
ser monitoradas sozinhas ou misturadas em um mosaico de forma transparente ao usuário. Utilizando
um conjunto de ferramentas especiais, permite a detecção de movimento ao vivo e o controle automático
de qualidade de imagem.
O Cliente de Monitoramento ainda permite a interoperabilidade entre as edições do sistema.
Atenção
Não é recomendável a execução do Cliente de Monitoramento no mesmo computador execurando o
módulo de servidor, exceto para pequenas instalações, pois o processamento utilizado pelo Cliente de
Monitoramento para a exibição das câmeras na tela poderá prejudicar as gravações realizadas pelo
sistema. Isso se deve ao fato que o Cliente de Monitoramento necessita decodificar as imagens para
exibição na tela, e dependendo do número de câmeras em tela esse processamento pode ser alto.
2.1 Como executar o Cliente de Monitoramento
Para acessar o Cliente de Monitoramento localize na sua Área de Trabalho o ícone Cliente de
Monitoramento ou menu de programas.
Ao ser executado a seguinte tela deverá aparecer:
O Cliente de Monitoramento 15
© 2002 - 2024 por Digifort. Todos direitos reservados.
2.2 Interface do Cliente de Monitoramento
O Cliente de Monitoramento foi projetado para possuir uma interface simples e intuitiva, onde operadores
com um mínimo de treinamento poderão operar o sistema de forma fácil e eficiente. A interface do
sistema é composta por diversos elementos e ferramentas. Veja abaixo seus principais elementos:
1. Menu de Opções
2. Painel de Visualização de Câmeras e Objetos
3. Painel de Informações
4. Seleção de Painel de Controle (PTZ, Privacidade, Audio)
5. Painel de Controle selecionado (PTZ, Privacidade, Audio)
6. Lista de Objetos
7. Lista de Layouts
8. Controle de Mosaicos
9. Controle de Bookmark
2.2.1 Menu de Opções
Para acessar o Menu de Opções, clique no triangulo no canto inferior da tela.
2.2.1.1 Botão Configurações
Abre a tela de configurações do Cliente de Monitoramento.
Para aprender a configurar o Cliente de Monitoramento, veja o capítulo de Configuração do Cliente de
Monitoramento.
2.2.1.2 Teclado Virtual
24
16 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Abre o teclado virtual.
O Teclado virtual possibilita o uso do Cliente de Monitoramento sem a necessidade do teclado físico
2.2.1.3 Botão Atualizar
Reinicia a conexão com os servidores.
2.2.1.4 Botão Tela Cheia
Expande o espaço reservado para a visualização da câmera de modo a preencher toda a tela. Para
voltar ao modo normal, pressione a tecla ESC do seu teclado.
2.2.1.5 Botão Minimizar
Minimiza o sistema na mesma barra onde se localiza o relógio do Windows (Bandeja).
2.2.1.6 Botão Desliga
Fecha o sistema.
2.2.1.7 Botão Reprodutor de Mídia
Abre o Reprodutor de Mídia onde você poderá selecionar qualquer câmera do sistema e visualizar os
vídeos gravados filtrados por data e hora.
Para aprender a reproduzir vídeos, veja o capítulo sobre Reprodução de Mídia .
2.2.1.8 Analítico
97
O Cliente de Monitoramento 17
© 2002 - 2024 por Digifort. Todos direitos reservados.
Este menu possui 2 submenus:
· Pesquisa de Registros: Abre a ferramenta de pesquisa de relatórios de eventos de analíticos. Para
aprender sobre a pesquisa de registros de analítico, consulte o capítulo sobre Pesquisa de Registros
de Analítico .
· Pesquisa de Metadados: Abre a ferramenta de pesquisa em metadados de analítico (Pesquisa
Forense). Para aprender sobre a pesquisa forese de metadados de analítico, consulte o capítulo sobre
Pesquisa de Metadados de Analítico .
2.2.1.9 LPR
Este menu possui 2 submenus:
· Pesquisa de Registros: Abre a ferramenta de pesquisa de registros e relatórios de LPR. Para
aprender sobre pesquisa de registros de LPR, consulte o capítulo sobre Registros de LPR .
· Zonas de LPR: Abre a ferramenta de pesquisa de registros e relatórios para o recurso de Zonas de
LPR. Para aprender sobre Zonas de LPR, consulte o capítulo sobre Zonas de LPR .
2.2.1.10 Botão Disparar Eventos
Abre a tela de disparo de eventos globais que pode ser, por exemplo, a abertura de uma tranca
eletrônica.
Para aprender a disparar eventos, consulte o tópico sobre Como Acionar Eventos Globais .
Para aprender a cadastrar e gerenciar eventos globais, consulte o manual do Cliente de Administração.
2.2.1.11 Gravações Protegidas
Abre a ferramenta de gerenciamento e visualização de gravações protegidas. Para aprender mais sobre
este recurso, consulte o tópico sobre Proteção de Gravações .
2.2.1.12 Log de Eventos
Abre a tela de pesquisa dos Logs de eventos. Para aprender sobre este recurso veja o capítulo Logs de
eventos .
2.2.1.13 Lista de alarmes locais
Abre ou fecha a lista de alarmes locais. Para aprender a utilizar este recurso, verifique o capítulo sobre
Lista de Alarmes Locais .
185
205
220
241
155
272
250
140
18 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
2.2.1.14 Auditoria
A tela de registro de auditoria possibilita pesquisar a auditoria de todos os servidores conectados
simultaneamente. Para aprender a utilizar este recurso, consulte o tópico sobre Registros de
Auditoria .
2.2.2 Painel de Visualização de Câmeras e Objetos
O painel de visualização de câmeras e objetos é o controle em formato de grade, onde você poderá
posicionar objetos como câmeras e mapas, para visualização simultânea. Este controle pode assumir
diversos layouts personalizáveis. Você poderá adicionar objetos nesta grid através da lista de objetos,
atalho de câmeras, dentre outros diferentes métodos que você aprenderá neste manual.
Uma vez que um objeto estiver na grade, você poderá arrastar ele, e trocar a sua posição com outro
objeto, ou arrastar para um espaço vazio, através da função de arrastar e soltar com o mouse.
Para maximizar um objeto em tela cheia, basta realizar um Duplo Clique no objeto, e para voltar ele para
o seu tamanho (e posição) original, utilize novamente o Duplo Clique. Alternativamente, você poderá
utilizar o atalho Shift + Clique no objeto.
Para remover um objeto da tela, clique com o botão direito do mouse sobre o objeto e o seu Menu de
Contexto será exibido, clique sobre a opção Remover Objeto. Alternativamente, você também poderá
arrastar e soltar o objeto sobre os controles de Layouts ou Mosaico:
2.2.3 Painel de Informações
264
O Cliente de Monitoramento 19
© 2002 - 2024 por Digifort. Todos direitos reservados.
O Painel de Informações irá fornecer dados vitais do sistema:
· Nome do Monitor: Exibe o nome, ou número do monitor, em sistemas com múlti-monitor. Este
nome pode ser utilizado para identificar o monitor quando múltiplas telas estão abertas.
· FPS: Exibe o total de Frames por Segundo que o sistema está exibindo (De todas as câmeras em
tela).
· CPU: Exibe o uso de CPU atual da estação de monitoramento.
· Data e Hora: Exibe a data e hora corrente da estação de monitoramento.
· Mem: Exibe o consumo de memória atual da estação de monitoramento.
2.2.4 Seleção de Painel de Controle
Este controle permite trocar entre diferentes tipos de painels de controle.
2.2.5 Painels de Controle
2.2.5.1 Controle de PTZ
Este controle visual permite a movimentação e controle de câmeras PTZ. Para aprender mais sobre este
controle, consulte o tópico sobre PTZ com Controles de Tela .
2.2.5.2 Modo de Privacidade
Este painel possui os recursos para controle do Modo de Privacidade. Para aprender mais sobre este
modo, consulte o tópico sobre Modo de Privacidade .
72
83
20 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
2.2.5.3 Áudio
Este painel possui os controles de áudio, que permitem receber e enviar áudio para câmeras ao vivo.
Para aprender mais sobre como receber e enviar áudio para câmeras, consulte o tópico sobre Áudio .
2.2.6 Lista de Objetos
A lista de objetos é um dos controles mais importantes da interface principal do Cliente de
Monitoramento. Nela será exibida todos os objetos aos quais o operador tem acesso e é através desta
lista que você poderá adicionar objetos em tela para visualização, reprodução, e diversas outras
funcionalidades. Para aprender a como utilizar este recurso, consulte o tópico Trabalhando com a Lista
de Objetos .
2.2.7 Lista de Layouts
Este controle permite a troca do layout do Painel de Visualização de Câmeras e Objetos. Para aprender
como utilizar, criar e excluir layouts, consulte o tópico Trabalhando com Layouts de Tela .
2.2.8 Controle de Mosaicos
Este controle fornece informações sobre o mosaico atual, assim como controles para criação, alteração
e exclusão de mosaicos. Para aprender mais sobre este recurso, consulte o tópico sobre Mosaicos de
Monitoramento .
80
56
64
67
O Cliente de Monitoramento 21
© 2002 - 2024 por Digifort. Todos direitos reservados.
2.2.9 Bookmarks
Este controle fornece acesso rápido para criação e pesquisa de bookmarks. Para aprender mais sobre
este recurso, consulte o tópico sobre Bookmarks .
2.2.10 Atalhos
Para facilitar o trabalho do operador, o Cliente de Monitoramento oferece alguns atalhos para acesso
rápido a alguns recursos mais utilizados.
2.2.10.1 Tecla F2
Exibe a opção para chamar uma câmera na tela pelo seu atalho (Configurado nas opções Gerais da
câmera, no Cliente de Administração).
2.2.10.2 Tecla F3
Tira uma foto da câmera selecionada e abre um diálogo com o a foto, e opções para salvar em disco.
Utilize Shift+F3 para salvar a imagem diretamente no diretório padrão de exportações, sem abrir a
janela de diálogo.
2.2.10.3 Tecla F4
Aciona a função de Revisão Instantânea para a câmera selecionada.
2.2.10.4 Tecla F5
Reconecta em todos os servidores configurados. Este atalho tem o mesmo efeito do botão Atualizar,
localizado no menu principal.
2.2.10.5 Tecla F11
Exibe as câmeras em tela cheia. Para sair do modo tela cheia pressione a tecla ESC. Este atalho tem
o mesmo efeito do botão Tela Cheia, localizado no menu principal.
Você também poderá sair de tela cheia através do menu de contexto, clicando com o botão direito do
mouse sobre a tela ou sobre um objeto, selecione a opção Sair da tela cheia.
2.2.10.6 Tecla F12
Altera a senha do usuário logado nos servidores conectados. Para aprender a utilizar este recurso veja o
capitulo Alterando a senha de usuário .
2.2.10.7 Ctrl + S
Enviar áudio para a câmera selecionada (O sistema irá enviar áudio enquanto o atalho estiver sendo
pressionado).
2.2.10.8 Ctrl + B
Criar um novo bookmark.
143
131
279
22 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
2.2.10.9 Ctrl + Y
Ativar / Desativar o Modo de Privacidade para a câmera selecionada.
2.2.10.10 Ctrl + F
Congela / Descongela a imagem ao vivo da câmera selecionada.
2.2.10.11 Ctrl + D
Ativar / Desativar o Zoom Digital para a câmera selecionada.
2.2.10.12 Ctrl + H
Chamar posição Home da câmera PTZ selecionada.
2.2.10.13 Ctrl + L
Bloquear / Desbloquear a câmera PTZ selecionada para uso exclusivo.
2.2.10.14 Ctrl + P
Pausar / Despausar Vigilância PTZ da câmera selecionada.
2.2.10.15 Ctrl + J
Ativar / Desativar Joystick Visual para a câmera selecionada.
2.2.10.16 Ctrl + 0..9
Chamar preset (0 a 9) da câmera selecionada.
2.2.10.17 Shift + Clique
Expande a área de visualização de um controle (Como uma câmera) que se encontra na grid de
controles, para tela cheia. Para voltar ao estado normal repita o mesmo processo.
C
h
a
p
t
e
r
I
I
I
24 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
3 Configurando o Cliente de Monitoramento
Esta área do sistema permite que você configure e personalize o Cliente de Monitoramento. Nas
configurações devem ser especificados os servidores que serão monitorados e os parâmetros de
ambiente do Cliente de Monitoramento.
Para acessar a área de configurações clique no botão de configurações, dentro do Menu de Opções.
Se você adicionar, alterar ou excluir qualquer servidor nas configurações do cliente, então você deverá
clicar no botão Atualizar para que o Cliente de Monitoramento reconecte nos servidores utilizando as
configurações desejadas. Qualquer outra alteração de configurações serão aplicadas quando um novo
mosaico ou câmera for selecionado na tela.
3.1 Configuraçoes Gerais
Esta é a tela de configurações gerais do Cliente de Monitoramento, ela lhe fornece as seguintes opções:
3.1.1 Lembrar o último mosaico selecionado ao abrir o sistema
Quando você seleciona um mosaico ou câmera, o sistema automaticamente guarda a sua referência
para que mais tarde caso o Cliente de Monitoramento seja reaberto a mesma câmera ou mosaico
reapareça automaticamente na tela. Se esta opção não estiver marcada, nenhuma câmera ou mosaico
será carregado automaticamente para exibição quando o Cliente de Monitoramento for aberto.
3.1.2 Lembrar o último mosaico selecionado ao trocar layouts
Quando esta opção está ativa, o sistema irá recarregar o último mosaico que foi visualizado para o estilo
de tela (Layout) selecionado. Se esta opção estiver selecionada, ela irá ter precedência sobre a opção
de Manter os objetos em tela ao trocar layout 25 .
Configurando o Cliente de Monitoramento 25
© 2002 - 2024 por Digifort. Todos direitos reservados.
3.1.3 Manter os objetos em tela ao trocar layout
Por padrão, quando você seleciona um novo layout, o sistema irá limpar a tela para que o layout novo
seja exibido vazio (a menos que a opção "Lembrar o último mosaico ao trocar de layouts" esteja
selecionada). Porém, com esta opção selecionada, o sistema irá manter a câmera, ou objetos visuais,
que já estejam em tela e popular o layout seguinte, permitindo assim ao usuário "abrir mais espaço"
para a criação de mosaicos. Caso o layout selecionado tenha menos espaços do que o número de
objetos em tela o sistema irá remover os objetos excedentes.
3.1.4 Inicializar o cliente de monitoramento na inicialização do sistema operacional
Inicia o Cliente de Monitoramento quando o sistema operacional iniciar automatizando o processo de
monitoramento das câmeras.
3.1.5 Esconder barras de ferramentas na inicialização
Expande o espaço reservado para a visualização da câmera de modo a preencher toda a tela ao iniciar o
sistema.
3.1.6 Esconder barra de ferramentas automaticamente com inatividade
Expande o espaço reservado para a visualização da câmera de modo a preencher toda a tela após um
tempo determinado de inatividade. O tempo de inatividade é contabilizando quando o operador do
sistema não estiver mexendo o mouse ou digitando no teclado da estação de monitoramento.
· Segundos de Inatividade: Define o tempo de inatividade para a tela ser expandida.
3.1.7 Barra de controles
Essa opção permite o posicionamento da barra lateral de controles na esquerda ou na direita do
monitor.
3.1.8 Barra de ferramentas
Essa opção permite colocar a barra de ferramentas na parte inferior ou superior do cliente de
monitoramento.
3.1.9 Tela
Essa opção permite a configuração do tipo de janela que o Cliente de Monitoramento será exibido:
· Janelada: O sistema se comportará como uma janela do Windows com as opções de minimizar,
maximizar e redimensionamento.
· Tela Cheia: O sistema ocupará toda a tela.
3.1.10 Tema
Permite a escolha da cor do tema do Cliente de Monitoramento.
3.1.11 Lista de objetos
É possível escolher como a identificação dos objetos aparecerão na lista de objetos no cliente de
monitoramento
· Nome e Descrição: Exibe na lista o nome e a descrição dos objetos.
· Somente o nome: Exibe na lista apenas o nome do objeto.
· Somente a descrição: Exibe na lista apenas a descrição do objeto.
28
26 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
3.1.12 Exibir objetos desativadas
Por padrão, os objetos desativados não serão exibidos na lista de objetos. Ative esta opção para exibir
os objetos desativados na lista de objetos.
3.1.13 Exibir apenas os mosaicos do estilo selecionado
Por padrão o sistema irá exibir apenas os mosaicos do layout selecionado na lista de objetos, porém ao
desativar esta opção, todos os mosaicos serão exibidos na lista de objetos, independente do layout
selecionado.
3.1.14 Auto expandir nós ao procurar objetos
Esta opção fará com que o sistema exiba os objetos na lista com os nós automaticamente expandidos
ao realizar uma busca.
3.1.15 Diretório de gravação local
O sistema dispõe da funcionalidade de realizar gravações locais de emergência nos computadores de
monitoramento.
· Diretório: Selecione o diretório para armazenamento dos vídeos gravados localmente.
· Formato de Gravação: Selecione o formato de gravação
o Formato Nativo: Gravação local em formato nativo (Só pode ser reproduzido utilizando o
Reprodutor de Vídeo)
o MP4: Gravação local em .mp4, compatível com reprodutores de vídeo comuns.
Nota
O usuário utilizado para rodar o Cliente de Monitoramento (Usuário do Sistema Operacional) deve ter
direito de gravar na pasta selecionada. A pasta padrão selecionada será a mesma pasta de instalação
do cliente, que geralmente é localizada dentro da pasta Arquivos de Programas, cujos usuários
normais do Sistema Operacional geralmente não possui direitos de escrita. Tenha certeza de selecionar
uma pasta com direitos de escrita.
Para aprender a realizar gravações locais veja o capitulo Realizando Gravações Locais .
3.1.16 Exportação e Screenshot
Define um diretório padrão para a salvar fotos da tela e vídeos exportados.
· Adicionar nome, data e hora nas imagens exportadas: Quando uma imagem ou vídeo for
exportado, o sistema irá renderizar o nome da câmera, data e hora da imagem. Esta opção poderá
deixar exportações mais lentas pois o sistema precisará fazer a transcodificação de vídeo durante a
exportação.
· Diretório padrão de exportação: Selecione o diretório padrão que será sugerido durante o processo
de exportação de vídeo.
Nota
O usuário utilizado para rodar o Cliente de Monitoramento (Usuário do Sistema Operacional) deve ter
direito de gravar na pasta selecionada. A pasta padrão selecionada será a mesma pasta de instalação
do cliente, que geralmente é localizada dentro da pasta Arquivos de Programas, cujos usuários
normais do Sistema Operacional geralmente não possui direitos de escrita. Tenha certeza de selecionar
uma pasta com direitos de escrita.
87
Configurando o Cliente de Monitoramento 27
© 2002 - 2024 por Digifort. Todos direitos reservados.
3.2 Configurando os servidores a serem monitorados
Esta é a tela de configurações de servidores. Nesta tela você poderá configurar quais servidores o
Cliente de Monitoramento irá monitorar.
Lembrando que a arquitetura do sistema é cliente-servidor, você pode adicionar quantos servidores
desejar, estando eles em sua rede local ou internet, assim o Cliente de Monitoramento irá monitorar
todos eles de forma única, como ser fosse um único servidor.
Temos as seguintes opções:
· Exibir lista de servidores para o usuário: Ao desmarcar esta opção, a lista de servidores não
estará visível para que o usuário possa conectar / desconectar de servidores específicos.
· Reutilizar o mesmo login para todos os servidores: Esta opção fará com que o mesmo login
(usuário/senha) seja utilizado ao tentar conectar em todos os servidores cadastrados.
· Disparar alarme local em caso de perda de conexão com servidor: Dispara um popup de
alarme quando a conexão com algum servidor for perdida.
· Utilizar operação com servidores de Failover: Esta opção deve ser marcada para que o usuário
não tenha objetos duplicados na operação com servidores de failover. Além disso quando esta opção
for utilizada o sistema irá repopular os objetos em tela durante o failover/failback, tornado a operação
transparente para o operador.
· Não exibir mensagens de alerta de servidores: Esta opção desabilita mensagens de alerta (como
licenciamento ou banco de dados) que aparecem ao logar em servidores, sendo particularmente útil
em aplicações onde o cliente de monitoramento está sendo exibido num videowall, sem que haja um
operador controlando.
· Intervalo de Ping: Este valor é utilizado para verificar se o servidor ainda está disponível e
funcionando, caso o servidor não responda a mensagem de ping do Cliente, então a conexão será
finalizada e reiniciada. Em caso de uso de servidores de Failover, utilize valores menores para o
28 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
cliente identificar mais rapidamente no caso do servidor principal se tornar indisponível, assim a
mudança para o servidor de Failover se dará mais rapidamente.
Nota
Ao adicionar, alterar ou remover um servidor, o Cliente de Monitoramento deve ser atualizado clicando no
botão Atualizar localizado no Menu de Opções.
3.2.1 Adcionando, alterando e removendo um servidor
Para adicionar um servidor clique sobre o botão Adicionar, e para alterar um servidor selecione o
servidor desejado na lista de servidores e clique sobre o botão Alterar ou se preferir dê um duplo clique
sobre o servidor. Para remover um servidor selecione o servidor desejado e clique em Remover.
A figura abaixo ilustra a tela de inserção ou alteração de servidores.
· Nome do servidor: Forneça um nome de identificação para o servidor. Após salvo, este nome não
poderá ser alterado, pois ele será usado para identificação do servidor no Cliente de Monitoramento.
· IP do Servidor: Preencha este campo com o endereço IP do servidor. Um endereço de DNS também
pode ser utilizado.
· Porta: Digite a porta de conexão com o servidor. A porta padrão é 8600 para conexões não seguras e
8400 para conexões seguras.
· Usar SSL: Selecione esta opção para utilizar conexão segura, criptografada com o servidor.
· Descrição: Digite uma breve descrição para o servidor, usada apenas para ajudar na sua identificação
no sistema por parte do operador.
· Auto Login: Esta opção habilita os campos usuário e senha para preenchimento. Habilitando esta
opção, sempre que o Cliente de Monitoramento for executado ou atualizado, ele fará a autenticação
no servidor utilizando o usuário e senha fornecidos. Caso esta opção esteja desmarcada, o usuário
deverá entrar com o seu usuário e senha na tela de login que irá aparecer quando o cliente se
conectar ao servidor.
· Método de conexão: Selecione o tipo de conexão com o servidor.
o Conexão Interna: Selecione Conexão Interna caso o servidor esteja na sua rede local. Com esta
opção seleciona, o cliente irá utilizar as configurações de IP Privado, para acesso direto às
câmeras (caso configurado).
o Conexão Externa: Selecione Conexão Externa caso o servidor ele esteja conectado via internet.
Com esta opção seleciona, o cliente irá utilizar as configurações de IP Público, para acesso direto
às câmeras (caso configurado).
· Método de receber mídia: Selecione o método de transmissão de mídia do servidor para o cliente:
o Unicast: Cada objeto em telá irá abrir uma nova conexão direta e independente com o servidor do
sistema. O vídeo será transferido através desta conexão.
Configurando o Cliente de Monitoramento 29
© 2002 - 2024 por Digifort. Todos direitos reservados.
o Multicast: Se o servidor tiver multicast habilitado, o Cliente poderá receber o vídeo através da
transmissão via multicast, a fim de economizar recursos de rede, pois se múltiplos clientes
estiverem recebendo stream da mesma câmera, este será enviado apenas uma vez na rede e
compartilhado com todos os clientes "conectados" neste stream. A transmissão via multicast
geralmente só irá funcionar em redes locais.
· Servidores: Exibe todos os servidores encontrados na rede. Selecione um registro da lista e os
campos IP do Servidor e Porta se preencherão automaticamente.
· Ativo: Demarque esta opção se deseja que o Cliente de Monitoramento não conecte nesse servidor.
Desmarcando esta opção nenhum objeto deste servidor estará disponível para visualização.
· Auto Conectar: Faz com que o Cliente de Monitoramento se conecte automaticamente ao servidor
quando iniciado ou atualizado.
Importante
Se a senha do usuário informado nos campos de auto-login for alterado pelo administrador no Cliente de
Administração ou alterado pelo próprio usuário através do modulo de troca de senha, os valores aqui
informados deverão ser atualizados.
Dica
Se o módulo de Servidor estiver executando no mesmo computador do Cliente de Monitoramento,
poderá ser utilizado o IP de Loopback identificado por 127.0.0.1.
3.3 Configurações do monitoramento ao vivo
Esta configuração se divide em três partes: Monitoramento ao Vivo, Informação de Câmeras e
Redimensionamento de Imagens.
3.3.1 Monitoramento ao Vivo
· Exibir barra de título das câmeras: Mostra uma barra preta onde as informações da câmera será
posicionada no topo da imagem.
· Exibir nome no título das câmeras: Exibe o nome da câmera no topo da imagem.
· Exibir descrição no título das câmeras: Exibe a descrição da câmera no topo da imagem.
· Exibir data no título das câmeras: Exibe a data atual no topo da imagem.
· Exibir hora no título das câmeras: Exibe o horário atual no topo da imagem.
· Fontes (Fonts): Opção para mudar a fonte com que as descrições das câmeras serão exibidas.
30 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Com todas opções anteriores ativas Com nenhuma opção anterior ativa
· Exibir mensagem de reconexão: Quando a comunicação com a câmera falhar por algum motivo,
se esta opção estiver habilitada, o Cliente de Monitoramento mostrará uma mensagem de reconexão:
· Exibir controles de gravação: Exibe os controles de gravação local, permitindo ao operador gravar
imagens das câmeras desejadas na própria estação de monitoramento para uma posterior
reprodução. Para aprender a realizar as gravações locais veja o capitulo Realizando Gravações
Locais .
· Parar a transmissão de câmeras sobrepostas em tela cheia: As câmeras podem ficar
sobrepostas quando o usuário seleciona uma câmera em tela cheia (Através do duplo clique). Neste
caso, todas as câmeras que estão em baixo (Não sendo exibidas) continuam a transmitir e
decodificar, mas ao ativar esta opção, o fluxo de vídeo destas câmeras sobrepostas será desativado,
economizando banda e recursos da CPU.
87
Configurando o Cliente de Monitoramento 31
© 2002 - 2024 por Digifort. Todos direitos reservados.
3.3.2 Informação de Câmeras
· Exibir taxa de quadros por segundo: Exibe na imagem da câmera a taxa de quadros por segundo
atualmente sendo recebida.
· Exibir resolução: Exibe na imagem da câmera, a resolução da imagem sendo exibida.
· Exibir taxa de transferência: Exibe na imagem da câmera a largura de banda utilizada pela câmera
localmente.
· Exibir decoder de vídeo utilizado: Exibe na imagem da câmera o decoder utilizado para a
decodificação e exibição da imagem na tela.
· Exibir status da conexão: Exibe na imagem da tela o status de transmissão.
3.3.3 Redimensionamento de Imagens
· Não redimensionar as imagens: As imagens provenientes das câmeras serão exibidas no seu
tamanho real, sem redimensionamento. Se a resolução da imagem for menor que o espaço reservado
para ela a imagem irá ficar pequena, e se a imagem for maior que o espaço reservado para ela
algumas partes dela serão perdidas. Este recurso é utilizado para exibir A figura abaixo ilustra o
funcionamento deste recurso.
· Redimensionar para preencher o espaço todo: As imagens provenientes das câmeras sempre
serão redimensionadas para que ocupem todo o espaço reservado para elas. A figura abaixo ilustra o
funcionamento desta função.
· Redimensionar mantendo a proporção da imagem: Redimensiona a imagem proveniente das
câmeras de modo que a largura e altura são redimensionadas proporcionalmente à area de
visualização.
32 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
· Utilizar redimensionamento bilinear: Quando as imagens das câmeras são redimensionadas,
algumas distorções podem ocorrer, como bordas serrilhadas. Habilitando este recurso as imagens
passarão por um filtro que minimiza esta distorção, mantendo a qualidade da imagem mais próxima
da imagem real.
Importante
Ao ativar o redimensionamento bilinear será exigido da estação de monitoramento um maior poder de
processamento, pois a correção de distorção de imagens é realizada através de algoritmos complexos e
intensivos no uso de CPU.
3.4 Configurações de Video / Áudio
3.4.1 Configurações de Video
· Buffer de Vídeo: Por padrão, o Cliente de Monitoramento não irá utilizará o buffer de vídeo, o que
significa que o vídeo das câmeras será renderizado instantaneamente ao ser recebido. Apesar de esta
ser a opção que oferece a visualização com menor atraso possível, o vídeo poderá não ficar suave o
suficiente pois a renderização depende de diversos fatores externos como a qualidade da transmissão
via rede, a câmera, carga no servidor de gravação, dentre outros. Ao utilizar o buffer de vídeo, o
sistema irá receber as imagens e manter alguns milissegundos em memória e então reproduzir as
imagens de maneira constante, aumentando muito a fluidez do vídeo, porém, este recurso irá
Configurando o Cliente de Monitoramento 33
© 2002 - 2024 por Digifort. Todos direitos reservados.
adicionar maior latência de exibição de vídeo, o que pode não ser operacionalmente viável para
câmeras PTZ, por isso o sistema permite a ativação do buffer de acordo com o tipo da câmera:
o Usar buffer de vídeo para câmeras fixas: Ativa o buffer de vídeo para câmeras fixas. Informe o
tamanho do buffer em milisegundos.
o Usar buffer de vídeo para câmeras PTZ: Ativa o buffer de vídeo para câmeras PTZ. Informe o
tamanho do buffer em milisegundos.
· Utilizar multi-thread para decodificação via software: O Cliente de Monitoramento permite o uso
de multi-thread para decodificação de vídeo H.264 e H.265. Esta opção pode ser utilizada para
acelerar a decodificação de vídeo no client, especialmente de imagens ultra megapixel. O uso desta
opção irá adicionar ao menos 1 frame de atraso no vídeo, ou seja, a 30 frames por segundo o atraso
adicional será de pelo menos 33ms enquanto a 7 frames por segundo o atraso adicional será de pelo
menos 143ms.
3.4.2 Configurações do Áudio
· Dispositivo de entrada de áudio: Escolha o dispositivo de captação de áudio. O sistema detectará
os dispositivos reconhecidos pelo Windows.
· Modo de Áudio:
o Half Duplex: Enquanto é enviado o som para a câmera não é possível escutar nenhum áudio vindo
da câmera.
o Full Duplex: É possível escutar e falar ao mesmo tempo.
· Tamanho do Buffer de áudio: Especifique o tamanho do buffer de áudio. Este buffer é necessário
para a correta reprodução do áudio recebido. Aumente este valor caso o audio recebido das câmeras
esteja quebrando.
3.5 Decodificação de Vídeo através de GPU
Para maior performance do sistema, é possível utilizar aceleração de decodificação de vídeo através de
placas de vídeo suportadas.
A decodificação de vídeo via GPU irá reduzir drasticamente o uso de CPU da estação de
monitoramento, permitindo um número maior de câmeras decodificando em paralelo.
34 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
3.5.1 NVidia
Para maiores informações sobre compatibilidade das placas de vídeo NVidia visite este link:
https://developer.nvidia.com/video-encode-decode-gpu-support-matrix#Decoder
A decodificação via GPU NVidia está disponível apenas no Cliente de Monitoramento de 64bits
(Surveillance64.exe) e requer a instalação dos drivers oficiais da NVidia:
https://www.nvidia.com/Download/index.aspx?lang=en-us
É possível utilizar 2 ou mais placas de vídeo para decodificação, e o sistema irá balancear a carga de
decodificação entre as placas de vídeo disponíveis.
O sistema ainda implementa um monitor de recursos da placa de vídeo, onde irá constantemente avaliar
o uso de GPU e o uso da memória da GPU. Caso o uso de GPU ou memória estiver acima do limite
configurado, o sistema irá fazer offloading e a decodificação via CPU (Software) será utilizada.
Cada instância de câmera sendo decodificada (Independente da resolução utilizada) utilizará cerca de
200~250MB de memória da GPU.
· H.264 Decoder: Ativa a decodificação de vídeo H.264 via GPU NVidia.
· H.265 Decoder: Ativa a decodificação de vídeo H.265 via GPU NVidia.
· Decodificação Paralela: O sistema permite paralelismo para decodificação de vídeo H.264 e H.265
via GPU. Esta opção pode ser utilizada para acelerar a decodificação de vídeo no client,
especialmente de imagens ultra megapixel. O uso desta opção irá adicionar ao menos 1 frame de
atraso no vídeo, ou seja, a 30 frames por segundo o atraso adicional será de pelo menos 33ms
enquanto a 7 frames por segundo o atraso adicional será de pelo menos 143ms.
· Uso Máximo de Memória: Selecione o valor máximo de uso de memória da GPU para
decodificação. Caso o uso máximo de memória seja atingido, novas câmeras serão decodificadas via
software.
o Padrão: Restaura o valor padrão para esta opção.
· Usar Gerenciador de uso de GPU: Ative esta opção para o sistema monitorar o uso da GPU e
iniciar o offloading de câmeras para decodificação via software caso o uso de GPU esteja acima do
limite configurado.
o Padrão: Restaura o valor padrão para esta opção.
3.5.2 Intel
O sistema também suporta decodificação de vídeo (H.264 e H.265) via QuickSync através da placa de
vídeo de processadores Intel. O QuickSync é uma tecnologia da Intel que possibilita a decodificação de
vídeo através do processador gráfico embutido em seus processadores. Para utilizar o QuickSync o
computador deve suportar o uso da placa de vídeo embutida (Intel HD Graphics) e a mesma deve estar
ativa no sistema operacional. É recomendado o uso do QuickSync para visualização de imagens de 5
megapixels ou superior, onde os ganhos da decodificação de hardware são mais notáveis. Também é
recomendado utilizar o Cliente de Monitoramento 64bits pois o uso de memória é maior.
· H.264 Decoder: Ativa a decodificação de vídeo H.264 via GPU Intel.
· H.265 Decoder: Ativa a decodificação de vídeo H.265 via GPU Intel.
Configurando o Cliente de Monitoramento 35
© 2002 - 2024 por Digifort. Todos direitos reservados.
3.6 Configurações de Reprodução de Vídeo
· Opções do Reprodutor de Mídia
o Zoom Inicial da Linha de Tempo: Especifique o zoom na qual a linha de tempo será exibida por
padrão
§ Padrão: Restaura o valor padrão para esta opção
· Revisão Instantânea: Selecione o modo de operação da Revisão Instantânea:
o Últimos 5 Segundos: Abre o reprodutor de vídeo para exibir os últimos 5 segundos gravados da
câmera selecionada.
o Últimos 10 Segundos: Abre o reprodutor de vídeo para exibir os últimos 10 segundos gravados da
câmera selecionada.
o Últimos 15 Segundos: Abre o reprodutor de vídeo para exibir os últimos 15 segundos gravados da
câmera selecionada.
o Últimos 20 Segundos: Abre o reprodutor de vídeo para exibir os últimos 20 segundos gravados da
câmera selecionada.
o Instantâneo com Reprodução para Trás: Abre o reprodutor de vídeo no horário atual e inicia a
reprodução em reverso.
36 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
3.7 Configurações da detecção de movimento
A detecção de movimento possibilita ao operador o reconhecimento mais fácil de movimento em uma
imagem.
A detecção de movimento é um filtro aplicado à imagem, realçando os movimentos da imagem na cor
desejada.
· Ativar detecção de movimento no cliente: Ativa o filtro de detecção de movimento.
· Cor do movimento: Selecione a cor de realce de movimento clicando sobre o controle de cor.
· Sensibilidade: Sensibilidade de reconhecimento de movimento.
· Padrão: Restaura o valor padrão para esta opção.
3.8 Configurações do Multi Monitor
O sistema dispõe do recurso de utilizar diversos monitores interligados em uma única estação de
monitoramento, criando uma tela de monitoramento individual em cada monitor onde é possível, por
exemplo, exibir um mosaico de monitoramento em um dos monitores, e uma única câmera nos outros.
Dessa maneira, adicionando diversas saídas de vídeo, o Cliente de Monitoramento trabalhará como um
decodificador e multiplexador de imagens para quantas câmeras forem necessárias.
Configurando o Cliente de Monitoramento 37
© 2002 - 2024 por Digifort. Todos direitos reservados.
· Ativar o suporte para multi monitor: Ativa o suporte ao multi monitor.
· Monitores detectados: Número de monitores detectados na sua estação de trabalho.
· Modo de Operação:
o Automático: Abre uma tela de monitoramento automaticamente em cada monitor reconhecido.
o Manual: Escolha a quantidade de telas de monitoramento a serem abertas. Você deverá posicionar
manualmente as telas em cada monitor. O sistema irá salvar a posição da tela e posicionará as
telas automaticamente na próxima vez que for aberto.
§ Total de monitores a utilizar: Selecione a quantidade de monitores a utilizar no modo manual.
· Definir um monitor padrão para o Reprodutor de Mídia: Define em qual monitor o reprodutor de
mídia do sistema irá aparecer ao ser aberto.
3.9 Alarmes
A tela de alarmes permite diversas configurações relacionadas com os pop-ups de alarmes que são
disparados no cliente de monitoramento.
38 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
3.9.1 Notificação de Alarme
· Janela de alarme com borda vermelha: Esta opção faz com que a borda do pop-up de alarme
fique vermelha.
· Fechar automaticamente a janela de notificação de alarme: Essa opção faz com que a janela
de alarme seja fechada automaticamente após um tempo configurado.
o Tempo para fechar: Configure o tempo (Em segundos) em que o pop-up de alarme irá fechar
automaticamente.
o Cancelar fechamento automático para janelas movidas pelo usuário: Caso o fechamento
automático esteja ativado, essa opção não deixará o pop-up fechar automaticamente caso o usuário
o arraste.
· Limitar a quantidade de janelas de alarmes simultâneas: O sistema de alarme permite limitar a
quantidade de janelas de alarme abertas simultaneamente. Quando o limite de janelas é atingido, o
popup mais antigo será fechado automaticamente.
o Janelas de alarmes simultâneas: Configure o número máximo de janelas de alarme simultâneas.
· Salvar e manter a posição da janela de alarme ao fechar: Esta opção irá salvar a posição da
janela de alarme no momento em que ela foi fechada pela última vez e irá utilizar esta posição para a
próxima janela de alarme que for aberta.
· Não remover o foco da janela atual para a nova janela de alarme: Com esta opção ativa, o
sistema não irá mover o foco da janela atual para a nova janela de alarme.
3.9.1.1 Auto Posicionar Janelas de Alarme
Permite que o sistema, por padrão, faça o auto ajuste da posição das janelas de alarmes nos
monitores. Quando um novo popup de alarme é aberto, o sistema irá automaticamente reposicionar e
ajustar o tamanho das janelas de alarme abertas nos monitores. Você poderá definir uma ordem de
monitores (para sistemas com múltiplos monitores), onde quando o limite de janelas abertas em um
monitor for atingido, o sistema continuará abrindo os alarmes no próximo monitor. O layout de
posicionamento de janelas pode ser definido individualmente para cada monitor, assim como o limite de
quantas janelas de alarme serão abertas em cada monitor.
Você deverá configurar a ordem dos monitores através da lista de cadastro:
Na imagem acima foi configurado a seguinte ordem: os primeiros 4 pop-ups irão aparecer no monitor
número 2 e os 4 outros serão mostrados no monitor de número 1. Nesse caso, se aparecer 9 pop-ups o
mais antigo será fechado, mantendo apenas os últimos 8.
Clique no botão Adicionar para adicionar a configuração de um novo monitor. Clique em Alterar para
alterar as configurações de um monitor e Excluir para excluir o registro deste monitor e remover ele da
rotação de janelas de alarme.
Configurando o Cliente de Monitoramento 39
© 2002 - 2024 por Digifort. Todos direitos reservados.
· Monitor: Selecione o número do monitor que receberá os alarmes.
· Total de alarmes: Selecione a quantidade máxima de janelas de alarme que este monitor poderá
exibir.
· Posicionamento das janelas
o Auto: Selecione automático para que o sistema arranje a posição dos alarmes automaticamente,
de acordo com o Total de alarmes configurado.
o Fixo: Selecione Fixo para configurar manualmente o layout desejado. Verifique o tópico sobre
Editor de Estilos de Tela para saber mais sobre como alterar o layout.
· Ordenação
o Novo alarme em primeiro: No caso hipotético de termos na tela o alarme A1 e A2, um novo
alarme tomaria o lugar do A1. Teríamos então: A1 (novo alarme), A2 e A3.
o Novo alarme um último: No caso hipotético de termos na tela o alarme A1 e A2, um novo alarme
seria o A3. Teríamos então: A1, A2 e A3 (novo alarme).
· Não reposicionar janela para eventos repetidos: Se um mesmo alarme for disparado multiplas
vezes vezes, o pop-up que já estiver aberto na tela, referente a este alarme, não será reposicionado,
caso contrário, a janela será movida para a primeira ou última posição na lista de alarmes (De acordo
com a configuração de ordenação, descrita anteriormente).
· Cancelar reposicionamento automático para janelas de alarme movidas pelo usuário: As
janelas que forem movidas pelo usuário serão removidas do gerenciador de posições de janelas de
alarme e não serão mais reposicionadas quando alarmes forem abertos ou fechados.
3.9.2 Lista de Alarmes Locais
· Número de dias para manter os alarmes
o Dias para alarmes em aberto: Número de dias que o sistema irá manter os alarmes que ainda
estão abertos, na lista.
o Dias para alarmes fechados: Número de dias que o sistema irá manter os alarmes que já estão
fechados, na lista.
· Cor dos alarmes abertos: Cor dos alarmes em aberto, para facilitar a leitura da tela. Clique no
controle de cor para alterar a cor.
· Cor dos alarmes fechados: Cor dos alarmes fechados, para facilitar a leitura da tela. Clique no
controle de cor para alterar a cor.
65
40 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
3.10 Mesa controladora
Esta tela permite a configuração de opções de operação de Mesa Controladora Digifort.
· Ativar beep ao pressionar tecla: Ativa e desativa o beep sonoro ao pressionar uma tecla da Mesa
Controladora.
· Taxa de repetição de teclas: Ajusta a velocidade de repetição ao manter uma tecla da Mesa
Controladora pressionada.
o Padrão: Restaura o valor padrão para esta opção.
· Velocidade máxima para aceleração na reprodução de vídeo: Define a velocidade máxima da
aceleração do video na reprodução de mídia ao girar o joystick da Mesa Controladora.
o Padrão: Restaura o valor padrão para esta opção.
· Usar a Mesa Controladora na reprodução de mídia: Permite ativar ou desativar o uso da Mesa
Controladora para reprodução de mídia. Quando o reprodutor de mídia for aberto, se esta opção estiver
ativada, você poderá controlar a reprodução com o joystick, e caso esteja desativada, a Mesa
Controladora continuará funcionando para a câmera selecionada ao vivo, enviando controles de PTZ
para esta câmera ao invés de comandar o reprodutor de mídia.
3.11 Configurações do Joystick
O sistema possibilita o uso de Joysticks comuns para o controle de PTZ das câmeras. Nesta tela você
poderá configurar as opções de Joystick.
Configurando o Cliente de Monitoramento 41
© 2002 - 2024 por Digifort. Todos direitos reservados.
Clique em Configurar Joystick para abrir a configuração de Joystick físico.
· Joystick Visual: O sistema possui um controle de Joystick Visual, que será sobreposto na imagem
da câmera, para simular o uso de Joystick em uma câmera através do mouse. Para aprender a operar
o Joystick Visual, consulte o tópico sobre Controle de PTZ com Joystick Visual .
o Tamanho dos botões de zoom: O tamanho do botão de zoom do Joystick Visual no Cliente de
Monitoramento pode ser aumentado em até 300% para otimizar o uso em tablets Windows.
3.11.1 Geral
· Ativar Joystick: Ativa o uso de Joystick físico para controle de PTZ de câmeras.
· Número do Joystick: Selecione o número do joystick (Configurado no Sistema Operacional).
· Abrir Configurações do Windows: Abre o gerenciador de joysticks do sistema operacional.
· Restaurar Padrão: Restaura os valores padrões para todas as configurações.
28
42 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
3.11.2 Botões
O sistema permite associar funções de PTZ para os botões do joystick. Aqui podem ser definidos os
botões que irão realizar as operações de zoom, foco, troca de câmeras, dentre outros.
· Menos zoom: Selecione o botão do joysitck que irá realizar a função de retração de zoom.
· Mais zoom: Selecione o botão do joystick que irá realizar a função de aproximação de zoom.
· Fator zoom: Selecione um fator de operação, em porcentagem. Esse fator se aplica à velocidade do
zoom. No exemplo acima a câmera irá movimentar seu zoom em 80% da sua velocidade máxima,
quando o botão for pressionado.
· Foco perto: Selecione o botão que irá realizar a função de ajuste de foco para perto.
· Foco longe: Selecione o botão que irá realizar a função de ajuste de foco para longe.
· Fator foco: Selecione um fator de operação, em porcentagem. Esse fator se aplica a velocidade de
ajuste do foco. No exemplo acima a câmera irá ajustar seu foco em 80% da sua velocidade máxima,
quando o botão for pressioado.
· Fechar íris: Selecione o botão que irá realizar a função de fechamento da íris.
· Abrir íris: Selecione o botão que irá realizar a função de abertura da íris.
· Fator íris: Selecione um fator de operação, em porcentagem. Esse fator se aplica a velocidade de
ajuste da íris. No exemplo acima a câmera irá ajustar sua íris em 80% da sua velocidade máxima,
quando o botão for pressioado.
· Ativar auto foco: Selecione o botão que irá realizar a função de auto foco.
· Ativar auto íris: Selecione o botão que irá realizar a função de auto íris.
· Mover esquerda: Selecione o botão que irá realizar a função de troca de seleção de câmeras. Este
botão seleciona a câmera mais próxima à esquerda da câmera selecionada.
· Mover direita: Selecione o botão que irá realizar a função de troca de seleção de câmeras. Este
botão seleciona a câmera mais próxima à direita da câmera selecionada.
· Mover cima: Selecione o botão que irá realizar a função de troca de seleção de câmeras. Este botão
seleciona a câmera mais próxima à cima da câmera selecionada.
· Mover baixo: Selecione o botão que irá realizar a função de troca de seleção de câmeras. Este
botão seleciona a câmera mais próxima a baixo da câmera selecionada.
3.11.3 Eixos
A configuração dos eixos permite que você escolha qual operação deseja atribuir para cada eixo do
joystick, sendo que o sistema permite cinco tipos de operações contínuas sendo elas: Pan, Tilt, Zoom,
Configurando o Cliente de Monitoramento 43
© 2002 - 2024 por Digifort. Todos direitos reservados.
Ajuste de foco e Ajuste de íris. Para atribuir as operações para os eixos do joystick, apenas selecione
a operação na caixa de seleção do eixo desejado. Cada operação só pode ser atribuída a apenas um
eixo.
· Operação do eixo X: Selecione a operação PTZ que o eixo X do joystick irá realizar.
· Operação do eixo Y: Selecione a operação PTZ que o eixo Y do joystick irá realizar.
· Operação do eixo Z: Selecione a operação PTZ que o eixo Z do joystick irá realizar.
· Operação do eixo R: Selecione a operação PTZ que o eixo R do joystick irá realizar.
· Operação do eixo U: Selecione a operação PTZ que o eixo U do joystick irá realizar.
· Operação do eixo V: Selecione a operaçãoqque o eixo V do joystick irá realizar.
· Inverter: Todos os eixos podem ser invertidos, ou seja, as operações de direita, esquerda, cima e
baixo serão invertidas.
3.11.4 Sensibilidade dos Eixos
A configuração de sensibilidade dos eixos permite que você divida a área de trabalho dos eixos em
varias partes iguais sendo que a sensibilidade aumenta de acordo com a quantidade de divisões. Você
pode escolher valores entre 1 a 30 para qualquer eixo. O padrão de operação são 10 divisões.
44 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
· Eixo X: Selecione sensibilidade do eixo X.
· Eixo Y: Selecione sensibilidade do eixo Y.
· Eixo Z: Selecione sensibilidade do eixo Z.
· Eixo R: Selecione sensibilidade do eixo R.
· Eixo U: Selecione sensibilidade do eixo U.
· Eixo V: Selecione sensibilidade do eixo V.
Dica
Ao alterar os valores de sensibilidade, os controles visuais para teste do joystick serão alterados para
refletir a divisão escolhida para cada eixo.
3.11.5 Margem Central dos Eixos
A configuração de margem dos eixos permite que o joystick trabalhe com uma folga central, isto se
torna necessário para alguns joysticks que não são muito precisos e retornam ao seu ponto central com
certa folga, por isso é necessário definir uma margem para a operação nula.
· Eixo X: Selecione a margem do eixo X.
Configurando o Cliente de Monitoramento 45
© 2002 - 2024 por Digifort. Todos direitos reservados.
· Eixo Y: Selecione a margem do eixo Y.
· Eixo Z: Selecione a margem do eixo Z.
· Eixo R: Selecione a margem do eixo R.
· Eixo U: Selecione a margem do eixo U.
· Eixo V: Selecione a margem do eixo V.
Dica
Ao alterar os valores de margem, os controles visuais para teste do joystick serão alterados para refletir
a margem central escolhida para cada eixo.
3.11.6 Testando as configurações
Durante o ajuste das configurações do Joystick, você poderá testar as opções alteradas com ajuda de
controles visuais com feedback em tempo real.
Os controles visual de Teste de Sensibilidade irão exibir, em tempo real, a divisão atual de sensibilidade
dos eixos e margem central de operação. Ao mover os eixos do Joystick, você verá a sua posição atual
sendo exibida em tempo real nos controles visuais. Com isso você poderá realizar um ajuste fino nas
configurações de cada eixo, de acordo com o seu Joystick.
O painel de feedback de operação do joystick irá fornecer dados em tempo real sobre o joystick
· Joystick
o Eixos: O escopo dos valores de eixo é de 0 a 1000, sendo 500 o ponto central.
o Botões: Botões atualmente pressionados do joystick.
· Operações PTZ: Exibe os valores escalados, de acordo com a sensibilidade de cada eixo. O painel
fornece a informação do valor mínimo e máximo de cada eixo, assim como o valor atual escalado.
46 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
3.12 Matriz Virtual
· Ativar Matriz Virtual: Ativa a Matriz Virtual para este client, fazendo com que os monitores
configurados aqui, façam parte da rede de monitores da Matriz Virtual.
· Lista de Monitores: Digite o nome do monitor para ser apresentado à rede de monitores da Matriz
Virtual. Você deverá especificar um nome que seja único para cada monitor. Você não pode repetir o
mesmo nome de monitor em diferentes clientes, caso isso ocorra, o monitor do cliente que conectar
primeiro no servidor será utilizado como parte da Matriz Virtual. Se você não digitar um nome para o
monitor, ele não fará parte da Matriz Virtual. O número de monitores disponíveis será igual a
quantidade de telas (GUIs) abertas, de acordo com as Configurações de Multi-Monitor da estação.
· Exibir informações de origem do objeto: Quando um objeto (como câmeras, mosaicos ou mapas)
é enviado por um usuário para outro monitor através da matriz virtual, é mostrado ao operador as
informações de origem daquele objeto como mostra a figura abaixo:
· Piscar a borda quando um objeto é enviado: Quando um objeto (como câmeras, mosaicos ou
mapas) é enviado por um usuário para outro monitor através da matriz virtual, o painel de informações
de origem do objeto irá piscar entre as cores vermelho e preto como mostram as figuras abaixo (Esta
opção é dependente da opção de Exibir informações de origem do objeto):
· Exibir o nome do monitor na tela principal: Permite exibir o nome do monitor da Matriz Virtual na
tela principal da aplicação ao invés do número do monitor.
3.13 Links Visuais de Objetos
Esta tela permite a personalização da operação dos links de objetos. Para aprender a operar os links de
objetos, consulte o tópico sobre Operação com Links de Objetos .
36
88
Configurando o Cliente de Monitoramento 47
© 2002 - 2024 por Digifort. Todos direitos reservados.
· Ativar links de objetos: Permite o controle da ativação ou não dos links nesta estação de
monitoramento.
· Transparência dos links: Permite o controle de opacidade das zonas e ícones, de maneira que seja
possível ver a imagem da câmera por trás dos ícones e zonas ou mantê-los com uma cor sólida. Você
pode definir valores de transparência independentes para zonas ou ícones.
o Padrão: Restaura o valor padrão para esta opção.
· Auto esconder links: Esta opção faz com que os links desapareçam da imagem após o tempo
configurado, re-aparecendo quando houver movimentação do mouse na imagem.
o Tempo: Tempo (em segundos) para esconder os links.
· Exibir links em objetos de analítico: Esta opção faz com que os links estejam disponíveis no
objeto de câmera, exibido através do objeto de Configuração de Analítico.
· Exibir links em objetos de LPR: Esta opção faz com que os links estejam disponíveis no objeto de
câmera, exibido através do objeto de Configuração de LPR.
3.13.1 Ações ao clicar em links
A operação dos links se faz através de atalhos, ou seja, ao clicar em um link, o sistema poderá tomar
uma ação, como substituir o objeto atual pelo objeto referenciado pelo link, ou abrir este objeto em uma
nova janela, por exemplo. O sistema permite o uso de alguns modificadores como Shift+Click e
Ctrl+Shift+Click. Abaixo você poderá especificar qual o atalho deverá ser utilizado para determinada
ação. O sistema permite a operação com 3 atalhos diferentes, sendo eles Click, Shift+Click e
Ctrl+Shift+Click. Configure os atalhos, para as ações desejadas:
· Subtituir o objeto corrente: Esta ação fará com que a câmera seja substituída no seu espaço atual
em tela, pelo objeto referenciado pelo link (caso o link leve a um objeto, se for um evento, a câmera
permanecerá na tela).
· Adicionar o objeto em um espaço vazio: Ao executar esta ação, o sistema irá adicionar o objeto
referenciado pelo link em um espaço vazio no mosaico, sem remover a câmera da tela.
· Abrir um popup com o objeto: Esta ação irá abrir um popup com o objeto referenciado pelo link,
mantendo a câmera em tela.
3.14 Navegador Web
Esta tela fornece opções para o controle de Navegador Web:
48 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
· Navegador Padrão: Selecione o tipo de navegador para ser utilizado:
o Chromium
o Microsoft® Edge
o Microsoft® Internet Explorer
· Segurança
o Ignorar Certificados Inválidos: Não exibir mensagem de certificado inválido.
3.15 Mapas
Esta tela permite a configuração de opções para o objeto de Mapa Sinóptico.
· Barra de Ferramenta Aberta por Padrão: Ative esta opção para sempre exibir a barra de
ferramenta do mapa aberta (Na parte superior do controle). Caso esta opção esteja desativada, a barra
será exibida fechada, fornecendo maior área de visualização do mapa.
· Tipo de Redimensionamento Padrão: Permite a seleção do tipo padrão de exibição do mapa:
o Sem Redimensionamento: Exibe o mapa no seu tamanho padrão, sem redimensionar os ícones.
Configurando o Cliente de Monitoramento 49
© 2002 - 2024 por Digifort. Todos direitos reservados.
o Esticar: Exibe o mapa, esticando o seu conteúdo para toda a área de visualização onde ele está
sendo exibido.
o Proporcional: Exibe o mapa, esticando o seu conteúdo para toda a área de visualização onde o
ele está sendo exibido, mantendo as proporções originais (Sem distorções).
3.16 Mapas Operacionais
Esta tela permite a configuração de opções para o objeto de Mapa Operacional.
· Barra de Ferramenta Aberta por Padrão: Ative esta opção para sempre exibir a barra de
ferramenta do mapa aberta (Na parte superior do controle). Caso esta opção esteja desativada, a barra
será exibida fechada, fornecendo maior área de visualização do mapa.
· Modo Noite Ativado por Padrão: Ative esta opção para que o mapa seja exibido com o modo noite
ativado.
3.16.1 Ações ao clicar em objetos
O mapa operacional permite abrir os objetos exibidos nele, como câmeras, configurações de analítico,
dentre outros. O sistema fornece diferentes ações para exibição do objeto ao clicar, e diferentes atalhos
para ativar uma ação. Os atalhos suportados são: Duplo Clique, Shift+Duplo Clique,
Ctrl+Shift+Duplo Clique. Associe o atalho desejado para cada ação:
· Subtituir o objeto corrente: Esta ação fará com que o mapa seja substituído no seu espaço atual
em tela, pelo objeto referenciado pelo link (caso o link leve a um objeto, se for um evento, o mapa
permanecerá na tela).
· Adicionar o objeto em um espaço vazio: Ao executar esta ação, o sistema irá adicionar o objeto
referenciado pelo link em um espaço vazio no mosaico, sem remover o mapa operacional da tela.
· Abrir um popup com o objeto: Esta ação irá abrir um popup com o objeto referenciado pelo link,
mantendo o mapa operacional em tela.
3.17 Analíticos
Esta tela permite a configuração de opções para o objeto de Configuração de Analítico.
50 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
· Exibir Lista de Eventos Aberta: Ative esta opção para sempre exibir a lista de eventos aberta (Na
parte inferior do controle). Caso esta opção esteja desativada, a lista de eventos será exibida fechada
por padrão, fornecendo maior área de visualização da câmera.
· Opções de Renderização: É possível configurar as opções padrão de renderização de metadados
de analítico. Quando um objeto de analítico for adicionado em tela, as opções definidas nestas
configurações serão utilizadas por padrão. O usuário ainda pode alterar as opções de cada objeto
manualmente através do menu de contexto clicando com o botão direito do mouse sobre o objeto de
Configuração de Analítico. Selecione as opções desejadas para renderização padrão.
3.18 LPR
Esta tela fornece opções para personalização dos controles de LPR.
3.18.1 Lista de Placas Reconhecidas
Estas opções permitem configurar o tipo de exibição de miniatura de placa, na lista de placas
reconhecidas (No lado esquerdo do controle de LPR).
Configurando o Cliente de Monitoramento 51
© 2002 - 2024 por Digifort. Todos direitos reservados.
· Mostrar imagem da placa gerada pelo sistema: Esta opção irá gerar uma representação digital da
placa, com os caracteres reconhecidos, apresentando cores diferentes para cada caractere,
dependendo da sua confiança de leitura (Se suportado pelo engine ou câmera).
· Mostrar recorte da imagem da câmera: Esta opção irá gerar uma pequena imagem com a placa
recortada da imagem original, em miniatura.
3.18.2 Detalhes da Placa Reconhecida ou Selecionada
Estas opções permitem configurar o tipo de exibição de miniatura de placa, nos detalhes de pesquisa
de registros de LPR.
· Mostrar imagem da placa gerada pelo sistema: Esta opção irá gerar uma representação digital da
placa, com os caracteres reconhecidos, apresentando cores diferentes para cada caractere,
dependendo da sua confiança de leitura.
· Mostrar recorte da imagem da câmera: Esta opção irá gerar uma pequena imagem com a placa
recortada da imagem original, em miniatura.
3.18.3 Filtro de Placas
Fornece opções para filtrar as placas exibidas no controle de Configuração de LPR (Ao vivo).
· Exibir apenas placas reconhecidas que estão registradas em alguma lista: Esta opção irá filtrar
os registros ao vivo das Configurações de LPR e irá apenas exibir os registros de placas que estejam
registradas em alguma lista de placa do sistema. O efeito deste filtro é apenas para a visualização ao
vivo, e não afeta a pesquisa de registros de LPR reconhecidos.
3.18.4 LPR Bridge
Opções da integração com o LPR Bridge.
· Exibir barra de progresso durante a consulta com o LPR Bridge: Esta opção exibirá uma
pequena barra de progresso durante a consulta do registro pelo LPR Bridge.
52 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
3.18.5 Zonas de LPR
Opções para o controle visual de Zona de LPR.
· Lista de Placas aberta por padrão: Ative esta opção para sempre exibir a lista de placas na zona
(Na parte esquerda do controle). Caso esta opção esteja desativada, a lista de placas será exibida
fechada por padrão, fornecendo maior área de visualização do dashboard da zona.
3.19 Evidence
Esta tela fornece as configurações para a integração com o Evidence
· Ativar integração com Evidence: Habilita a integração com o Evidence.
· Endereço do Servidor: Forneça o endereço do servidor do Evidence.
· Porta HTTP: Forneça a porta do servidor HTTP do Evidence.
· Porta FTP: Forneça a porta do servidor FTP do Evidence.
3.20 Video Synopsis
O módulo de Video Synopsis é utilizado para investigar as gravações através de sinopses de vídeos e
inteligência de vídeo. Esta tela permite a configuração da integração com o módulo Video Synopsis.
Configurando o Cliente de Monitoramento 53
© 2002 - 2024 por Digifort. Todos direitos reservados.
· Ativar integração Video Synopsis: Habilita a integração com o Video Synopsis.
· Video Synopsys V4: Selecione esta opção para a integração com a versão 4 do Video Synopsis.
o Cliente Video Synopsis: Selecione a pasta onde o Cliente Video Synopsis foi instalado.
· Video Synopsis V5: Selecione esta opção para a integração com a versão 5 do Video Synopsis.
o Endereço do Servidor: Digite o endereço do servidor do Video Synopsis.
o Porta: Digite a porta do servidor.
o Auto Login: Ative esta opção para o sistema realizar auto login no Video Synopsis com usuário e
senha pré-cadastrado.
§ Usuário: Usuário para auto login.
§ Senha: Senha para auto login.
3.21 Mensagem de Aviso Legal
O sistema permite a exibição de uma mensagem personalizada de aviso legal (Disclaimer) ao abrir o
Cliente de Monitoramento ou Cliente de Administração. O usuário deverá clicar em "Eu Concordo" para
poder utilizar o sistema, caso contrário, o cliente será fechado.
Para adicionar uma mensagem de aviso legal personalizado basta adicionar um arquivo chamado
"Disclaimer.htm" na pasta de instalação dos clients.
54 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
3.22 Importando e Exportando Configurações
O sistema permite Importar e Exportar configurações do Cliente de Monitoramento facilmente. Na tela
de Configurações, utilize os botões Importar ou Exportar.
Ao clicar em um botão para Importar ou Exportar configurações, você poderá escolher entre
Configurações e Servidores:
· Configurações: Importa ou Exporta todas as configurações para um arquivo do tipo .ini.
· Servidores: Importa ou Exporta o cadastro de servidores para um arquivo do tipo .ini.
C
h
a
p
t
e
r
I
V
56 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
4 Lista de Objetos
A lista de objetos é um dos controles mais importantes da interface principal do Cliente de
Monitoramento. Nela será exibida todos os objetos aos quais o operador tem direitos de acesso, e você
irá utilizar esta lista para visualizar, reproduzir e acessar rapidamente diversas outras funcionalidades
através do menu de contexto com o botão direito do mouse.
Este controle é composto por 3 abas:
· Objetos: Lista de objetos do sistema.
· Monitores: Lista de monitores da Matriz Virtual.
· Servidores: Lista de servidores.
4.1 Filtros
Utilize a barra de pesquisa para filtrar registros. O termo digitado irá filtrar todos os objetos e irá exibir
apenas os objetos que possuem o termo digitado em seu nome ou descrição, como mostra a figura
abaixo:
4.2 Objetos
A aba Objetos irá fornecer a lista completa de todos os objetos que o operador possui direito de acesso
dos servidores conectados. Os objetos de todos os servidores serão exibidos em uma única lista
combinada.
4.2.1 Adicionando Objetos em Tela
Para adicionar um objeto no Painel de Visualização de Objetos, clique 2 vezes sobre o objeto
desejado na lista, e ele será adicionado em um espaço vazio no Painel de Visualização. Você
também poderá utilizar a função de arrastar e soltar:
Lista de Objetos 57
© 2002 - 2024 por Digifort. Todos direitos reservados.
Se você arrastar e soltar um objeto em um espaço vazio, este objeto irá preencher este espaço. Você
também poderá arrastar um objeto da lista de objetos, para um espaço onde outro objeto já está
ocupando, e neste caso, o objeto em tela será substituído pelo novo objeto:
O sistema também permite arrastar e soltar objetos no Reprodutor de Vídeo, facilitando a troca de
câmeras durante uma investigação de gravações:
Você também poderá arrastar um mosaico salvo para o Painel de Visualização ao Vivo, ou para o
Reprodutor de Vídeo.
58 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
4.2.1.1 Arrastando Grupos de Objetos
O sistema permite arrastar e soltar grupos de objetos, para o Painel de Visualização ao Vivo ou para
o Reprodutor de Vídeo.
Quando um grupo de objeto é arrastado, todos os objetos do grupo serão adicionados na tela:
No exemplo acima, ao arrastar o grupo Break Room, as câmeras 19 e 20 serão adicionadas em tela.
Por padrão, ao arrastar um grupo, apenas os objetos diretos do grupo serão adicionados em tela, assim
sendo, nenhum objeto de subgrupo será adicionado. Na figura acima, se o grupo First Floor for
arrastado, apenas a câmera 40 será adicionada em tela. Para arrastar objetos do grupo e todos os
objetos de todos os subgrupos deste grupo, segure a tecla Shift ao arrastar o grupo. No exemplo
acima, ao arrastar o grupo First Floor, segurando a tecla Shift, as câmeras 19, 20 e 40 serão
adicionadas em tela.
Se um filtro de objetos estiver aplicado, este também será aplicado ao arrastar grupos. Exemplo:
Na figura acima, o filtro break aplicado irá filtrar todos objetos com este nome. Como este é o nome de
um subgrupo, todos os objetos deste subgrupo serão exibidos. Note que a câmera 40, que pertence ao
grupo First Floor, não está sendo exibida. Neste exemplo, ao arrastar o grupo First Floor, com a tecla
Shift apertada (Para arrastar os objetos de subgrupos), apenas as câmeras 19 e 20 serão exibidas em
tela, e a câmera 40, mesmo pertencendo ao grupo First Floor, não será exibida pois ela está sendo
excluída pelo filtro.
4.2.2 Status das câmeras
O sistema utiliza diferentes ícones para representar uma câmera Fixa ou uma câmera
PTZ. O ícone de uma câmera Dome indica que a câmera em questão possui recursos
de PTZ ativados.
Estes ícones representam que a câmera está desativada.
Estes ícones representam que a câmera está ativada, em funcionamento, mas não está
escrevendo em disco no momento.
Lista de Objetos 59
© 2002 - 2024 por Digifort. Todos direitos reservados.
Estes ícones representam que a câmera está ativada, em funcionamento, e escrevendo
em disco no momento.
Estes ícones representam que a câmera está ativada, em funcionamento, detectando
movimento, porém não está escrevendo em disco. A detecção de movimento apenas
será sinalizada se a camera estiver gravando por movimento, ou com algum alarme de
movimento configurado.
Estes ícones representam que a câmera está ativada, em funcionamento, detectando
movimento, e escrevendo em disco. A detecção de movimento apenas será sinalizada se
a camera estiver gravando por movimento, ou com algum alarme de movimento
configurado.
Estes ícones representam que a câmera está fora de funcionamento.
4.2.3 Agrupamento de Ícones
Os ícones da lista de objetos poderão ser agrupados por Tipos de Objetos e por Servidores:
· Tipos de Objetos: Esta é a organlização padrão da lista, onde todos os objetos de todos os
servidores, serão agrupados pelo seu tipo, fornecendo uma visão geral e unificada para o usuário:
· Servidores: Na organização por servidores, os objetos de cada servidor estarão agrupados dentro de
um ícone específico de cada servidor:
60 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
No tipo de organização por servidores, os Mosaicos serão exibidos de maneira global, pois eles são
compartilhados com todos os servidores.
4.2.4 Menu de Contexto
O menu de contexto da lista de objetos é uma ferramenta muito poderosa para acesso rápido às
funções mais utilizadas para o tipo de objeto selecionado. Com atalhos de fácil entendimento e estrutura
simples e lógica, o menu de contexto irá otimizar o tempo de uso do sistema, fornecendo fácil acesso à
diversas ferramentas e recursos do sistema.
Para acessar o menu de contexto, clique com o botão direito do mouse sobre um objeto da lista:
Um menu com as opções disponíveis para o tipo de objeto selecionado será exibido. Os itens do menu
são atalhos para diversas funções do sistema como Reprodução de Vídeo, Matriz Virtual,
Bookmarks, Pesquisas, Mapas, dentre outros.
4.3 Monitores
A aba Monitores irá exibir a lista de monitores atualmente conectados na Matriz Virtual, fornecendo
informações sobre cada monitor:
Lista de Objetos 61
© 2002 - 2024 por Digifort. Todos direitos reservados.
Cada ítem de monitor da lista irá exibir as seguintes informações:
· Objetos em Tela: Nome do objeto que está sendo exibido atualmente em tela. Se um mosaico
estiver sendo exibido, o nome do mosaico será apresentado, assim como a lista de todos os objetos
dele.
· IP: IP da estação onde o monitor se encontra.
· Usuário: Usuário logado na estação onde o monitor se encontra.
A lista de monitores também apresenta um menu de contexto, acessível através do clique com o botão
direito do mouse:
Através deste menu de contexto você poderá remover os objetos do monitor selecionado. Esta operação
irá remover todos os objetos em tela.
Você também poderá enviar objetos para este monitor, através da função de arrastar e soltar. Para
aprender a utilizar a Matriz Virtual, consulte o tópico Matriz Virtual .
4.4 Servidores
A aba Servidores irá exibir uma lista com todos os servidores cadastrados no Cliente de
Monitoramento. Aqui você poderá verificar o status de conexão com os servidores, conectar e
desconectar de servidores:
160
62 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
O ícone ao lado esquerdo do servidor irá representar o seu status de conexão:
Servidor desconectado.
Servidor conectado.
Conexão em progresso.
Erro ao conectar no servidor.
Para conectar ou desconectar de um servidor, utilize o Duplo-Clique sobre o ícone do servidor desejado:
Se um erro ocorrer durante a conexão com o servidor, uma mensagem de error será exibida na coluna
Status:
Você pode ordenar a lista, clicando sobre a coluna desejada para ordenação.
C
h
a
p
t
e
r
V
64 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
5 Layouts e Mosaicos
O Cliente de Monitoramento permite exibir múltiplos objetos (Câmeras, Mapas...) em tela para
monitoramento. Estes objetos são adicionados em um Painel de Visualização de Câmeras e
Objetos :
O Painel de Visualização de Câmeras e Objetos pode assumir diferentes layouts (Estilo da Tela)
para a melhor organização dos objetos em tela. O sistema possui alguns layouts padrão e também
permite a criação de novos layouts:
Além dos layouts comuns, o sistema possui 2 tipos de layout especiais:
Automático: Este layout permite a criação de mosaicos de dimensionamento automático,
ou seja, podem ser inseridas neste mosaico quantos objetos forem necessárias e assim o
mosaico automaticamente redimensiona o tamanho do espaço reservado para cada objeto
de modo que todos possam ser exibidos na tela simultaneamente. Este layout sempre irá
crescer simétricamente (2x2, 3x3, 4x4, etc...).
Timer: Este layout permite a criação um seqüenciamento de objetos e mosaicos que
serão exibidos na tela intercaladamente com um tempo de espera definido pelo usuário.
Para aprender a utilizar este recurso veja o tópico sobre Mosaicos de Timer .
Para trocar o layout atual, basta clicar no ícone do novo layout desejado no controle. Caso a lista de
layouts seja muito extensa, clique nos ícones das setas para direita e esquerda para mover a lista de
layouts.
Após a seleção do layout desejado, você poderá adicionar os objetos em tela, e salvar este
posicionamento atual de objetos em um Mosaico (Ou Visão), com isso, você poderá recarregar esta
visão de objetos no futuro, apenas recarregando o mosaico salvo. Para aprender mais sobre como
trabalhar com mosaicos, veja o tópico sobre Mosaicos de Monitoramento .
Ao trocar o layout, o sistema poderá exibir automaticamente o último mosaico salvo para o novo layout,
caso a opção de Lembrar o último mosaico ao trocar de layouts esteja ativada. Se esta opção estiver
desativada, o sistema poderá Manter os objetos em tela ao trocar layout caso esta segunda opção
estiver ativada, caso contrário, o sistema irá limpar a tela, removendo todos os objetos, ao trocar o
layout.
18
68
67
28
25
Layouts e Mosaicos 65
© 2002 - 2024 por Digifort. Todos direitos reservados.
5.1 Criando Layouts
O Cliente de Monitoramento possue um editor de layouts embarcado. Para acessá-lo basta clicar nos
botões em sua barra de ferramentas:
Para adicionar um novo layout, clique no botão "+".
Para apagar um layout, selecione o layout e clique no botão "-". O sistema irá pedir uma confirmação
para remover o layout selecionado. O botão estará desabilitado para os layouts nativos do sistema, já
que esses não podem ser removidos.
Ao clicar no botão "+", a seguinte tela será exibida:
· Dimensão da Matriz: Escolha a dimensão da matriz a ser criada. O valor é NxN.
Selecione a dimensão da matriz e clique no botão Criar Matriz:
66 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Na figura acima criamos uma matriz 4x4, possibilitando a adição de 16 câmeras na tela.
Depois de criada a matriz, é possível unir quadrantes, clicando com o botão esquerdo do mouse e
arrastando-o, objetivando obter uma área maior de visualização, no exemplo acima estamos unindo os
quadrantes 1, 2, 5 e 6, formando o layout apresentado na figura abaixo:
Com a união desses quatro quadrantes obtemos espaço para alocação de 13 objetos, sendo que uma
delas vai ter o tamanho quatro vezes maior.
É possível unir quantos quadrantes forem necessários desde que a área final seja um retângulo.
Para desfazer uma união, repita o mesmo processo com o botão direito do mouse.
Clique no botão OK para salvar o Layout ou Cancelar para cancelar a operação.
Layouts e Mosaicos 67
© 2002 - 2024 por Digifort. Todos direitos reservados.
5.2 Criando Mosaicos
O sistema permite salvar o posicionamento atual dos objetos em tela em uma Mosaico, com isso, você
poderá recarregar esta visão de objetos no futuro, apenas recarregando o mosaico salvo.
· Os mosaicos são sempre categorizados de acordo com o seu layout, ou seja, você poderá criar e
salvar mosaicos para diferentes layouts.
· Você deverá escolher um nome único para este mosaico dentro da sua categoria (Layout).
· O sistema permite mosaicos com o mesmo nome, desde que eles possuam layouts diferentes.
· Não é possível criar mosaico para o layout de 1 objeto.
· Por padrão, o sistema irá exibir apenas os mosaicos do layout selecionado na Lista de Objetos , a
menos que a opção Exibir mosaicos apenas do layout selecionado seja desmarcada.
Para criar um mosaico de monitoramento, selecione o estilo de tela desejado, coloque os objetos em
tela desejados e clique sobre o botão de Salvar Mosaico:
A seguinte tela será exibida, com as opções para salvar o mosaico:
· Nome: Informe um nome para o mosaico. Este nome deve ser único, para o layout selecionado.
· Opções: As opções selecionadas aqui serão salvas juntamente com o mosaico, assim quando ele for
carregado novamente, estas opções salvas serão aplicadas.
o Salvar perfil de mídia selecionado: Selecionando esta opção, o Perfil de Mídia corrente de
cada câmera será salvo.
o Salvar configurações da detecção de movimento: Selecionando esta opção, as configurações
de detecção de movimento serão salvas juntamente com o mosaico. Para aprender a configurar a
detecção de moviemnto veja o capitulo Configurações da detecção de movimento .
o Salvar configurações dos filtros de imagem: Selecionando esta opção, as configurações de
filtros de imagem serão salvas juntamente com o mosaico. Para aprender a configurar os filtros de
imagem veja o capitulo Filtros de imagem .
o Salvar posição atual do zoom Digital: Selecionando esta opção, a ultima posição do zoom
digital deixada em cada câmera será salva. Para aprender sobre zoom digital veja o capitulo PTZ
Digital .
o Salvar posição atual de câmeras fisheye / panomorph: Selecionando esta opção, o filtro, modo
e posição atual do zoom no dewarp de câmeras 360 serão salvos. Para aprender sobre lentes
Fisheye / Panomorph, veja o capítulo Lentes Fisheye / Panomorph .
· Mosaico Público: Selecione esta opção para indicar que este é um mosaico público, ou deselecione
para indicar que ele é um mosaico privado. O mosaico público será exibido para todos os usuários do
sistema, enquanto o mosaico privado ficará restrido ao usuário que o criou.
56
26
86
36
84
79
93
68 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
O Mosaico Público será exibido na lista de objetos com o seguinte ícone:
O Mosaico Privado será exibido na lista de objetos com o seguinte ícone:
Os mosaicos de monitoramento serão atualizados dinamicamente em tempo real quando criados,
atualizados ou apagados em todos os clients, sem a necessidade de reconexão com o servidor.
Para excluir um mosaico, selecione o mosaico desejado e clique sobre o botão Deletar:
O botão Limpar, representado por uma lixeira, irá remover todos os objetos em tela.
5.2.1 Mosaicos de Timer
Este layout permite a criação um seqüenciamento de objetos e mosaicos que serão exibidos na tela
intercaladamente com um tempo de exibição definido pelo usuário. Para acessar este recurso selecione
o mosaico de timer na lista de layouts e em seguida clique em Novo Mosaico de Seqüenciamento,
conforme ilustrado na figura abaixo:
Após esse processo a tela de inclusão de mosaicos de timer será exibida, conforme ilustrado na figura
abaixo:
Layouts e Mosaicos 69
© 2002 - 2024 por Digifort. Todos direitos reservados.
· Nome do mosaico: Digite um nome de referência para o mosaico.
· Mosaicos disponíveis: Lista de mosaicos disponíveis para adicionar no mosaico de timer. Estes
mosaicos já devem estar previamente criados.
· Lista de Objetos: Lista de objetos disponíveis para adicionar no mosaico de timer.
· Configuração do seqüenciamento: Lista ordenada dos itens do mosaico de timer que serão
sequenciados na tela do Cliente de Monitoramento.
· Modificar: Modifica o tempo de exibição do ítem selecionado.
· Botões para cima e para baixo: Altera a ordem de exibição do objeto selecionado.
· Mosaico Público: Selecione esta opção para indicar que este é um mosaico público, ou deselecione
para indicar que ele é um mosaico privado. O mosaico público será exibido para todos os usuários do
sistema, enquanto o mosaico privado ficará restrido ao usuário que o criou.
Para adicionar mosaicos ao seqüenciamento, selecione-o e clique no botão Adicionar, representado
pela fecha vermelha apontada para a direita, ou para adicionar objetos, selecione o objeto e arraste para
a lista. Ao adicionar um objeto ou mosaico na lista de sequenciamento, a tela de solicitação do tempo
de exibição em que este objeto ou mosaico ficará na tela será exibida, conforme ilustrado na figura
abaixo:
· Segundos: Tempo de exibição do objeto ou mosaico selecionado
Informe o tempo desejado e clique em OK.
Após a adição de todos os mosaicos e/ou objetos desejados no mosaico de timer, clique em OK para
salvar e ele será exibido em tela, e iniciará a sua execução, sequenciando os itens na ordem que foi
criado na lista de sequenciamento.
70 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
O sistema fornece alguns controles de manipulação deste mosaico, localizados na tela principal do
Cliente de Monitoramento, com funções como pausar o seqüenciamento, avançar e retroceder entre
objetos ou mosaicos e reiniciar, conforme ilustrado na figura abaixo:
Este controle será apresentado abaixo da lista de objetos na tela principal do Cliente de Monitoramento,
apenas quando o layout TIMER for selecionado.
C
h
a
p
t
e
r
V
I
72 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
6 Câmeras
O controle de câmera é o componente mais importante do sistema, é com este controle que você
poderá visualizar uma câmera ao vivo:
Você pode personalizar e configurar este controle através das Configurações de Monitoramento Ao
Vivo do Cliente de Monitoramento.
O controle de câmera possui diversos atalhos de teclado. Veja o capítido de Atalhos do Cliente de
Monitoramento para aprender mais.
Com o botão direito do mouse, você poderá acessar um poderoso menu de contexto, com diversos
atalhos para otimizar a operação do sistema. Consulte o capítulo sobre Menu de Contexto para
aprender sobre os atalhos de câmera.
Nesta sessão você aprenderá como utilizar todos os recursos que o controle de câmera oferece.
6.1 PTZ
Através do Cliente de Monitoramento é possível controlar câmeras móveis através do recurso PTZ.
O sistema fornece várias formas de controle de movimentação de uma câmera:
· Controles Visuais da Tela
· Joystick Físico ou Mesa Controladora
· Joystick Visual
· Clicar e Centralizar
· Zoom em Área
· Zoom Digital
O funcionamento dos quatro formas de movimentação de uma câmera será explicado nos tópicos
seguintes.
6.1.1 Movimentação pelos controles da tela
O sistema fornece todas as ferramentas necessárias para a movimentação da câmera através dos
controles da tela, para acessar esse recurso localize os controles de PTZ na tela principal do Cliente de
Monitoramento, conforme ilustrado abaixo. Estes controles somente estarão disponíveis se uma câmera
com suporte PTZ estiver selecionada.
29
21
85
Câmeras 73
© 2002 - 2024 por Digifort. Todos direitos reservados.
Para movimentar uma câmera, primeiramente é preciso selecioná-la, para isso clique sobre a imagem
da câmera desejada. Uma borda colorida será exibida ao redor da câmera para indicar que ela está
selecionada.
6.1.1.1 Setas Direcionais
Move a câmera selecionada para a direção desejada. Se o driver da câmera selecionada suportar PTZ
por Joystick, você poderá clicar e segurar o botão direcional e a câmera irá mover enquanto o botão
estiver pressionado. Se o driver da câmera selecionada não suportar PTZ por Joystick, ao clicar nos
botões direcionais, a câmera irá mover alguns passos e parar, independentemente se você manter o
botão apertado.
6.1.1.2 Botões de Zoom
Realiza função de Mais Zoom ou Menos Zoom. Se o driver da câmera selecionada suportar PTZ por
Joystick, você poderá clicar e segurar os botões de zoom e a câmera irá realizar a função de zoom
enquanto o botão estiver pressionado. Se o driver da câmera selecionada não suportar PTZ por Joystick,
ao clicar nos botões de zoom, a câmera irá realizar alguns passos de zoom e parar, independentemente
se você manter o botão apertado.
6.1.1.3 Barra de Sensibilidade
Esta barra define a velocidade em que a câmera se movimentará ao realizar uma função de PTZ. O
sistema sempre guardará a última posição utilizada pelo usuário.
6.1.1.4 Botão Foco
Se a câmera suportar controle de foco, utilize estes controles para controlar o seu foco.
· Botão +: Ajusta o foco para objetos perto da câmera.
74 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
· Botão -: Ajusta o foco para objetos longe da câmera.
· Botão Auto Foco: Ativa ou desativa o Auto Foco. Ao pressionar este botão, o menu de contexto
abaixo será exibido, com as opções para Ativar ou Desativar o Auto Foco:
6.1.1.5 Botão Íris
Se a câmera suportar controle de íris, utilize estes controles para controlar a abertura e fechamento da
lente.
· Botão +: Abre a íris da lente para receber mais luz.
· Botão -: Fecha a íris da lente para receber menos luz.
· Botão Auto Íris: Ativa ou desativa o Auto Íris. Ao pressionar este botão, o menu de contexto abaixo
será exibido, com as opções para Ativar ou Desativar o Auto Íris:
6.1.1.6 Botão de PTZ Digital
Ativa ou desativa o PTZ Virtual. Para aprender o que é e como utilizar este recurso veja o tópico
Movimentação através do PTZ Digital .
6.1.1.7 Botão Joystick
Ativa ou desativa o joystick visual. Para aprender o que é e como utilizar este recurso veja
Movimentando através do Joystick Visual .
6.1.1.8 Botão de Bloqueio de PTZ
Bloqueia os controles de PTZ da câmera para uso exclusivo do operador, obedecendo à hierarquia de
prioridades pré-definidas pelo administrador. Ao ativar este bloqueio, apenas o operador que ativou o
bloqueio poderá controlar o PTZ da câmera, todos os outros operadores perderão o controle desta
câmera. Apenas um operador com maior prioridade poderá tomar o controle para ele.
O bloqueio do PTZ também pode ser feito através do atalho Ctrl + L.
Quando uma câmera PTZ estiver bloqueada para uso exclusivo, um ícone indicando o bloqueio será
exibido no controle de PTZ:
79
78
Câmeras 75
© 2002 - 2024 por Digifort. Todos direitos reservados.
Ao manter o mouse sobre este ícone, o sistema exibirá o nome do usuário e IP da estação que possui o
bloqueio.
6.1.1.9 Botão de Posição Home
A câmera se movimentará para a posição Home ao pressionar este botão.
A posição home também poode ser chamada através do atalho Ctrl + H.
6.1.1.10 Limpador de pára-brisa
Ativa o limpador de pára-brisa, caso a câmera tenha suporte para este recurso.
6.1.1.11 Presets
O controle de presets permite chamar os presets da câmera (Posições pré definidas) ou criar novos
presets (Caso o operador possua direito).
6.1.1.11.1 Chamando Presets
Para chamar um preset basta seleciona-lo na lista e clicar no botão play como mostra a figura abaixo:
O sistema também fornece acesso rápido para os primeiros 9 presets, através dos botões numerados:
Ao manter o mouse sobre um botão numerado, o sistema irá exibir o nome do preset.
76 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Você também pode chamar os presets através dos atalhos Ctrl + 0..9.
6.1.1.11.2 Criando Presets
Para gravar um preset, movimente a câmera para a posição desejada, e clique sobre o ícone de criar
salvar preset, representado por um botão Rec:
A janela de cadastro de presets será exibida:
· Número: Selecione o número do preset. O sistema irá auto incrementar este valor, sugerindo um
número de preset, baseado no valor do último preset cadastrado.
· Descrição: Forneça uma descrição para este preset.
Ao finalizar, clique no botão OK para salvar ou Cancelar para abortar a operação.
6.1.1.12 Vigilância PTZ
Este controle permite iniciar ou pausar uma Vigilância PTZ. Este é um recurso que permite a câmera
seguir uma ronda pré-definida pelo administrador.
Para iniciar uma Vigilância, selecione a rota desejada e clique sobre o botão Play.
Para pausar uma Vigilância, pressione o botão Stop.
Você também pode iniciar ou pausar uma Vigilância através do atalhos Ctrl + P.
6.1.1.13 Auxiliar
Ativa ou Desativa uma função auxiliar da camera se for suportado.
Para ativar ou desativar uma função auxiliar, primeiramente selecione a função no menu e pressione o
botão I para ativar ou O para desativar.
Câmeras 77
© 2002 - 2024 por Digifort. Todos direitos reservados.
6.1.1.14 Status do uso do PTZ
Quando algum usuário está utilizando o PTZ da câmera selecionada, um ícone indicando o seu uso será
exibido no controle de PTZ:
Ao colocar o mouse em cima do ícone é possível visualizar qual usuário está interagindo com PTZ da
câmera selecionada:
6.1.2 Movimentação através do Clicar e Centralizar
Este recurso, se suportado pelo driver da câmera, possibilita que o usuário clique sobre o ponto da
imagem da câmera em que deseja centralizar a imagem.
Ao clicar em um ponto da imagem, a câmera se movimentará se posicionando de forma a centralizar o
ponto clicado.
Para utilizar este recurso, selecione câmera desejada, e em seguida clique sobre o botão central no
controle de PTZ de tela. O botão central do controle ficará acionado:
Com este botão pressionado (Ativado), clique no ponto desejado na imagem da câmera.
Enquanto este botão estiver pressionado, o sistema não irá permitir a seleção de outra câmera. Desative
o botão para poder selecionar outra câmera.
78 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
6.1.3 Movimentação com Zoom em Área
O sistema permite a movimentação de uma câmera PTZ através da seleção de uma área da imagem,
caso o driver da câmera suporte esta função. Ao selecionar uma área na imagem da câmera, o sistema
irá posicionar a câmera para realizar o zoom na área desejada, movendo o Pan, Tilt e Zoom da câmera,
simultâneamente, a fim de centralizar e aproximar a área selecionada:
Para realizar a função de Zoom em Área, clique com o botão direito do mouse na posição inicial, e,
mantendo o botão direito pressionado, arraste o mouse para criar uma área de seleção. Ao soltar o
botão direito do mouse, o sistema irá realizar o zoom em área.
6.1.4 Movimentação através do Joystick Visual
O joystick visual é uma ferramenta que simula o funcionamento de um joystick de mesa através do
mouse.
Para ativar o joystick visual selecione uma câmera e em seguida clique sobre o botão Joystick:
Você também pode ativar e desativar o Joystick Visual através do atalho Ctrl + J.
Os controles de Joystick deverão aparecer como demonstra a figura abaixo:
Para utilizar o joystick visual, clique sobre a imagem com o botão esquerdo do mouse, mantenha o
botão pressionado, e movimente o mouse para qualquer posição da imagem. Quanto mais afastado no
centro da imagem o mouse estiver, mais rápido vai ser a movimentação da câmera, e vice-versa.
Para realizar operações de zoom, utilize a roda do mouse, girando-a para frente, a imagem será
aproximada, e para trás, a imagem será afastada. Você também pode utilizar os botões visuais + e –,
exibidos próximo ao centro da imagem. A velocidade do zoom também pode ser controlada e
Câmeras 79
© 2002 - 2024 por Digifort. Todos direitos reservados.
visualizada pelo controle ao lado esquerdo na imagem. Quanto mais longe do centro a marcação
vermelha estiver, mais rápido o será o zoom, e vice-versa.
6.1.5 Movimentação através do Joystick Físico
Se você possuir um joystick de mesa padrão USB ou uma Mesa Controladora, é possível realizar
operações de PTZ em uma câmera através dele.
Para aprender a configurar um Joystick USB, consulte o tópico sobre Configuração de Joystick .
Para aprender a configurar uma Mesa Controladora, consulte o tópico sobre Configuração de Mesa
Controladora .
Selecione a câmera desejada e utilize o Joystick para controlar o seu PTZ.
6.1.6 Movimentação através do Zoom Digital
O sistema permite realizar a função de Zoom Digital na imagem de câmeras fixas ou móveis.
Câmeras fixas possuem o Zoom Digital ativado por padrão sempre, para utilizar esta função em câmeras
PTZ, selecione a câmera desejada e clique sobre o botão de Zoom Digital:
Você também pode ativar e desativar o Zoom Digital através do atalho Ctrl + D.
O ícone representando uma lupa será exibido no controle de PTZ, indicando que o Zoom Digital está
ativado:
Com a função de Zoom Digital ativada, clique com o botão direito do mouse na posição inicial, e,
mantendo o botão direito pressionado, arraste o mouse para criar uma área de seleção. Ao soltar o
botão direito do mouse, o sistema irá realizar o zoom digital na área selecionada. O Zoom Digital
também pode ser feito pela roda do mouse ou pelos botões de zoom do controle de PTZ.
Ao realizar o zoom, uma mini imagem da câmera será exibida, com a marcação da área atual de
visualização do Zoom Digital em vermelho:
40
40
80 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Com o zoom iniciado, todas as opções de Pan e Tilt estarão habilitadas, e você poderá utilizar todos os
modos de operar PTZ para mover a posição do Zoom (Por exemplo via Joystick Físico, Controle Visual,
Joystick Visual, etc...).
Você também poderá utilizar a mini imagem, e realizar o Zoom Digital selecionando uma área dentro da
mini imagem, e o novo zoom será feito para a área selecionada.
Através da mini imagem, você poderá arrastar a seleção atual do zoom para outra área, para isso, clique
com o botão esquerdo do mouse sobre um ponto da mini imagem, a seleção atual do zoom será
centralizada nessa área, mantendo o botão esquerdo do mouse pressionado você poderá mover esta
seleção para outro ponto da imagem.
Para remover o zoom digital, clique duas vezes com o botão esquerdo do mouse, dentro da mini
imagem. Você também pode efetuar a operação de Menos Zoom até o zoom ser removido por completo.
6.2 Audio
O sistema permite comunicação de duas vias com as câmeras, ou seja, permite escutar o áudio das
câmeras em tempo real e também enviar áudio utilizando um microfone conectado na estação de
monitoramento.
Para acessar os controles de Áudio, clique no botão correspondente, no Painel de Seleção de
Controles:
O controle de áudio será exibido, conforme a figura acima.
Câmeras 81
© 2002 - 2024 por Digifort. Todos direitos reservados.
6.2.1 Escutar
Para ouvir o áudio de uma câmera, simplesmente selecione a câmera desejada na caixa de seleção:
Se a opção Auto-Selecionar estiver ativada, ao clicar sobre uma câmera ao vivo, no Painel de
Visualização de Câmeras e Objetos , a câmera será selecionada automaticamente no controle de
seleção de câmeras para receber audio.
· Botão Mute: Silencia o áudio da câmera.
· Barra de volume: Mostra o volume do áudio recebido, em tempo real.
6.2.2 Falar
Algumas câmeras permitem que possa ser enviado áudio para seus alto-falantes, ou seja, o operador
pode falar através da câmera.
Para falar, basta selecionar a câmera na caixa de seleção e clicar no botão abaixo:
Para falar, você deverá manter o botão acima pressionado. Você também poderá clicar duas vezes
sobre este botão e manter ele pressionado indefinitivamente, inclusive podendo trocar entre câmeras, ou
grupos de áudio, enquanto o áudio está sendo enviado.
Se a opção Auto-Selecionar estiver ativada, ao clicar sobre uma câmera ao vivo, no Painel de
Visualização de Câmeras e Objetos , a câmera será selecionada automaticamente no controle de
seleção de câmeras para receber audio.
6.2.2.1 Grupos de Dispositivos de Saída de Áudio
É possível enviar áudio para múltiplas câmeras ao mesmo tempo, ou seja, setorizar as áreas de áudio,
para isso, o sistema permite a criação de Grupos de Dispositivos de Saída de Áudio.
Para criar um grupo de dispositivos para o envio do áudio, basta clicar no botão com o sinal de + e a
janela abaixo se abrirá:
18
18
82 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
· Nome: Nome do grupo de dispositivos de saída de áudio.
· Descrição: Descrição do grupo criado
· Dispositivos: Clique em Adicionar para inserir os dispositivos no grupo, ou Excluir para remover os
grupos selecionados na lista.
Ao clicar no botão Adicionar, a tela de seleção de dispositivos (Com suporte a saída de áudio) será
exibida, e você poderá selecionar os dispositivos que deseja fazer parte do grupo:
Para alterar um grupo criado, selecione-o e clique no botão: -
Para excluir um grupo criado, selecione-o e clique no botão: x
6.2.3 Listar apenas as câmeras em tela
Câmeras 83
© 2002 - 2024 por Digifort. Todos direitos reservados.
Selecione esta opção para que as caixas de seleções contenham apenas as câmeras que estão
atualmente em tela, caso contrário, todas as câmeras serão listadas.
6.2.4 Volume
Ajuste o volume de seu alto-falante e do microfone arrastando as barras azuis mostradas na imagem
abaixo:
6.3 Troca Automática de Perfil de Mídia
O sistema possui uma função que permite a troca do perfil de mídia ao vivo, ao selecionar uma câmera
no Painel de Visualização de Câmeras e Objetos . Este recurso é especialmente útil para
economizar processamento e banda utilizada pela Estação de Monitoramento. Quando múltiplas
câmeras estão sendo visualizadas em um mesmo monitor, raramente será necessário utilizar a
resolução máxima para estas câmeras multiplexadas em tela, pois a resolução do monitor, geralmente
é menor do que a resolução de todas as câmeras combinadas, por isso, é recomendável utilizar uma
resolução menor para visualização ao vivo por padrão, e quando maiores detalhes é necessário, ter a
habilidade de trocar para um perfil de mídia com resolução maior facilmente.
Para manter a operação do sistema simples e eficiente, a troca do perfil de mídia pode ser feita apenas
selecionando a câmera ao vivo:
Note que no exemplo acima, a câmera não selecionada está exibindo uma imagem de resolução
704x480, que é própria para ser exibida em pequenos quadrantes, juntamente com outras câmeras. Por
ser uma imagem de baixa resolução, ela irá consumir menos banda e recursos de CPU ou GPU para
decodificação do vídeo, permitindo assim um número maior de câmeras simultâneas em tela. Quando a
câmera é selecionada (Imagem da direita), o sistema irá trocar para um perfil maior (No exemplo acima,
1920x1080), permitindo a visualização de maiores detalhes, assim como um melhor Zoom Digital .
Este recurso deve ser configurado pelo Administrador do Sistema através do Cliente de Administração.
6.4 Modo de Privacidade
O Modo de Privacidade permite determinar uma lista de usuários que irão perder o acesso à imagem de
uma câmera quando ele for ativado no Cliente de Monitoramento. Este recurso pode ser muito útil
quando as câmeras de uma instalação estiverem disponíveis externamente, com isto, o operador poderá
bloquear temporariamente o acesso externo à câmera quando desejar.
18
79
84 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Para acessar o controle de Modo de Privacidade, selecione o botão correspondente na Seleção de
Paineis de Controle:
Para ativar o Modo de Privacidade de uma câmera, selecione a câmera que deseja controlar e clique
sobre o botão de ativação do modo, exemplificado na imagem acima.
Após ativado, um ícone indicando que o Modo de Privadade está ativo para a câmera selecionada será
exibido no Controle de Modo de Privacidade, assim como a imagem será congelada para os usuários
configurados, com uma mensagem como mostra a imagem abaixo:
Os usuários que não tiverem configurados para manter o acesso à câmera não poderão ver suas
imagens ao vivo, e também não poderão reproduzir vídeos enquanto o Modo de Privacidade estiver
ativado:
6.5 Filtros de Imagem
Os Filtros de Imagem são configurações aplicadas à imagem de uma câmera objetivando realçar cores
e detalhes de uma cena para o auxilio de sua análise.
Para acessar esse recurso, clique com o botão direito do mouse sobre a imagem de alguma câmera,
exibindo assim o seu Menu de Contexto, e selecione a opção Filtros de Imagem, conforme ilustrado
na figura abaixo:
Câmeras 85
© 2002 - 2024 por Digifort. Todos direitos reservados.
· Vermelho: Ajusta o nível de cor vermelho da imagem.
· Azul: Ajusta o nível de cor azul da imagem.
· Verde: Ajusta o nível de cor verde da imagem.
· Contraste: Ajusta o nível de contraste da imagem.
· Brilho: Ajusta o nível de brilho da imagem.
· Nível de cor: Ajusta o nível de cor da imagem.
· Flip: Inverte a imagem horizontalmente. Recomendável quando a câmera esta instalada invertida.
· Flop: Inverte a imagem verticalmente. Recomendável quando a câmera esta instalada invertida.
· Escala de Cinza: Deixa a imagem em tons de cinza.
· Inverter: Inverte os canais de cores da imagem.
· Sharpen: Aplica na imagem o efeito de realce de bordas.
· Desentrelaçamento: O filtro de Desentrelaçamento suaviza as imagens que por causa do movimento
ficam com uma qualidade inferior. Este efeito geralmente ocorre em câmeras analógicas antigas, em
resolução 4CIF. Na figura abaixo possui um exemplo de desentrelaçamento.
· Botão Padrão: Retorna todos os valores para a posição padrão.
6.6 Menu de Contexto
Com o botão direito do mouse, você poderá acessar um poderoso menu de contexto da câmera, com
diversos atalhos para otimizar a operação do sistema.
86 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
6.6.1 Reprodução de mídia
O menu fornece diversas opções para rápida reprodução de mídia, como Revisão Instantânea e
Reprodução Rápida de câmera única e múltiplas câmeras.
Para aprender sobre a reprodução rápida de mídia veja o capítulo: Reprodução de vídeo rápida
6.6.2 Criar Bookmark
Abre a tela de criação de Bookmark, com a câmera escolhida.
Esta opção também será exibida no Menu de Contexto de objetos de câmeras ou grupo de câmeras na
Lista de Objetos . Caso esta opção seja executada a partir de um grupo de câmeras, todas as
câmeras do grupo serão adicionadas no Bookmark.
Para aprender sobre Bookmark veja o capítulo Bookmark .
6.6.3 Matriz Virtual
Envia o objeto para outro monitor através da Matriz Virtual. Para aprender sobre Matriz Virtual veja o
capítulo Matriz Virtual
6.6.4 Perfil de mídia
Selecionando este item, um sub-menu com todos os perfis de mídia da câmera selecionada será
exibido. Para alterar o perfil de mídia a ser utilizado no monitoramento da câmera basta selecionar a
opção desejada. Para aprender a criar perfis de mídia consulte o Manual do Cliente de
Administração.
131
20
143
160
Câmeras 87
© 2002 - 2024 por Digifort. Todos direitos reservados.
6.6.5 PTZ
Fornece acesso rápido para algumas opções de PTZ para a câmera. Para aprender sobre os recursos
de PTZ veja o capítulo PTZ .
6.6.6 Eventos Manuais
Caso haja eventos manuais cadastrados para essa câmera é possível ativa-los clicando em cima do
evento desejado. Para aprender sobre Eventos Manuais veja o capítulo Eventos Manuais .
6.6.7 Foto de tela
Selecionando este item, uma tela será exibida com a imagem atual da câmera selecionada, permitindo
salvar esta imagem em um arquivo.
6.6.8 Detecção de movimento
Selecionando este item, um sub-menu será exibido com as configurações da detecção de movimento
para a câmera selecionada:
· Ativar / Desativar: Ativa ou desativa a detecção de movimento para a câmera selecionada.
· Configurações: Abre a tela de configuração da detecção de movimento da câmera selecionada. Para
aprender a configurar a detecção de movimento veja o capitulo Configurações da detecção de
movimento .
6.6.9 Filtros de imagem
Abre a tela de configuração dos filtros de imagem para a câmera selecionada. Para aprender a
configurar os filtros de imagem veja o capitulo Como configurar os filtros de imagem .
6.6.10 Congelar Imagem
Ao clicar nesta opção o sistema irá congelar a imagem da câmera para que ela fique pausada. Para
voltar ao normal basta clicar na mesma opção novamente.
6.6.11 Pesquisa de Metadados de Analítico
Abre a tela de pesquisa de metadados de analítico com filtro de câmera já aplicado para exibir os
resultados apenas desta câmera.
6.6.12 Localizar em mapas
Esta opção dá a possibilidade de localizar a câmera selecionada em todos os mapas do tipo
selecionado que o usuário tem acesso. Caso a câmera faça parte de algum mapa o sistema abrirá uma
tela com os mapas em questão e a câmera denotada por um círculo vermelho.
· Mapa Operacional: Localiza a câmera em mapas operacionais.
· Mapa: Localiza a câmera em mapas sinópticos.
6.7 Realizando Gravações Locais
O sistema possibilita ao operador, a realização de gravações em sua estação de monitoramento, ou
seja, além das imagens serem gravadas no servidor, elas também serão gravadas no computador do
operador.
Para acessar este recurso, habilite os controles de gravação nas Configurações do Cliente de
Monitoramento .
72
157
36
84
29
88 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Após ativar este recurso, os controles de gravação serão exibidos sobre a imagem das câmeras,
conforme ilustrado na figura abaixo:
· Para iniciar a gravação da câmera na estação de trabalho do operador, clique sobre o controle de
gravação. Feito isso o controle ficará no estado piscante.
· Para parar a gravação, clique novamente sobre o controle de gravação.
As gravações locais desta câmera serão realizadas no Diretório de Gravação Local. Para alterar este
diretório, assim como o formato de gravação, consulte as Configurações do Cliente de
Monitoramento .
Para aprender a reproduzir vídeos locais, veja o capitulo sobre Reprodução de Vídeo Local .
6.8 Operação com Links de Objetos
A função de Links de Objetos fornece uma nova e revolucionária maneira de navegar entre as câmeras
do sistema, facilitando e acelerando a operação do sistema.
Os links de objetos permitem criar ligações virtuais entre diferentes câmeras ou objetos, e também a
criação de acionadores de eventos sobrepostos nas imagens das câmeras.
A imagem a seguir exibe um exemplo de utilização de links de objetos. Cada câmera em exibição
possui um link para outras câmeras na imagem. Ao clicar no link (Representado aqui por setas semi
transparentes), a câmera associada será carregada, permitindo a navegação rápida entre câmeras,
como por exemplo, ao seguir uma pessoa que está transitando entre as câmeras.
Também é possível associar eventos (E diversos outros tipos de objetos) nas imagens, como por
exemplo Eventos Globais que podem ser utilizados para acionar saídas de I/O para abrir portas e
portões. Na imagem abaixo, as câmeras 01 e 03 possuem botões para fisicamente abrir os portões.
26
135
Câmeras 89
© 2002 - 2024 por Digifort. Todos direitos reservados.
Os links de objetos também podem ser utilizados durante a reprodução de vídeo, tornando-se uma
ferramenta indispensável para a análise de incidentes gravados.
Os links de objetos podem ser representados por ícones, como exibido nas imagens acima ou também
por uma zona, que é representada através de um polígono semi-transparente na imagem, que pode ser
adicionada por exemplo, no contorno de uma porta ou portão, fornecendo uma representação visual de
que se o operador clicar neste portão, ele poderá ver a imagem da câmera que está do outro lado, ou
também poderá abrí-lo.
A imagem abaixo mostra uma zona de cor branca, que está associada a uma porta, ao clicar na porta,
a câmera de dentro da sala será exibida.
90 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
6.8.1 Modos de Operação
A função de Links de Objeto é muito flexível, e permite diversas formas de acessar e executar os links.
O sistema permite configurar um atalho para cada tipo de operação através das Configurações de Links
de Objeto .
O sistema permite diferentes tipos de ações ao executar um link, e fornece diferentes atalhos para
executar os tipos de ações disponíveis. Veja a seguir os diferentes modos de operar os links de objetos.
6.8.1.1 Substituir o objeto corrente
Esta ação fará com que a câmera seja substituída no seu espaço atual em tela, pelo objeto referenciado
pelo link (caso o link leve a um objeto, se for um evento, a câmera permanecerá na tela):
No exemplo acima, ao executar o link da câmera superior direita, através do seu atalho configurado
(Padrão Clique), a câmera ligada será exibida, substituindo a câmera original em seu espaço.
Caso o link seja para um Mosaico Público, todos os objetos em tela serão removidos, e o mosaico
referenciado será carregado.
6.8.1.2 Adicionar o objeto em um espaço vazio
Ao executar esta ação, o sistema irá adicionar o objeto referenciado pelo link, em um espaço vazio do
Painel de Visualização de Câmeras e Objetos, sem remover a câmera da tela. Caso não haja mais
espaços vazios em tela, o sistema irá abrir um Popup com o objeto referenciado.
46
Câmeras 91
© 2002 - 2024 por Digifort. Todos direitos reservados.
No exemplo acima, ao executar o link da câmera superior esquerda, através do seu atalho configurado
(Padrão Shift + Clique), a câmera ligada será adicionada no próximo espaço vazio do painel.
Caso o link seja para um Mosaico Público, todos os objetos em tela serão removidos, e o mosaico
referenciado será carregado.
6.8.1.3 Abrir um popup com o objeto
Ao executar esta ação, o sistema irá abrir um popup com o objeto referenciado pelo link.
No exemplo acima, ao executar o link da câmera superior esquerda, através do seu atalho configurado
(Padrão Ctrl + Shift + Clique), um popup com a câmera referenciada será aberto.
6.8.1.4 Arrastar e soltar um link para um espaço em tela
Permite arrastar um link de dentro de uma câmera, para o Painel de Visualização de Câmeras e
Objetos, e abrir o objeto correspondente do link, no quadrante escolhido.
No exemplo acima mostra um link da câmera superior esquerda, sendo arrastado para um espaço vazio,
e a câmera associada sendo aberta neste espaço.
Você também poderá arrastar um link sobre um outro objeto, substituindo este objeto de destino pelo
objeto do link.
6.8.2 Acionando Eventos ou Ações
Os Links de Objetos também podem referenciar eventos, como Eventos Globais e Eventos Manuais
de câmeras e ações como Chamar um Preset. Estes eventos geralmente estão associados com
ações físicas, como por exemplo abrir um portão, disparar um alarme ou acionar algum procedimento de
automação e o link fornece um acesso rápido e intuitivo para disparar este evento.
92 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Os Links para Eventos são sempre acessados através do atalho de Clique com o botão esquerdo do
mouse.
Na imagem a seguir, temos um link sobre um portão, referenciando a sua abertura. O objetivo deste link
é abrir o portão.
Ao clicar no link, o sistema irá exibir uma mensagem de confirmação para a execução do evento
associado. Você poderá suprimir essa mensagem, e executar o evento diretamente através do atalho
Shift + Clique. A mensagem de confirmação será exibida apenas para os objetos do tipo Evento.
Ações como Chamar um Preset serão executadas diretamente.
A figura abaixo representa a execução do evento, efetivamente abrindo o portão.
6.8.3 Reprodução de Vídeo
Os Links de Objetos são particularmente úteis e poderosos durante uma investigação com a
Reprodução de Mídia. Com eles você também terá todo o poder de trocar entre câmeras e mosaicos,
permitindo operações como por exemplo, seguir uma pessoa suspeita, que trafega em diversas
câmeras:
Câmeras 93
© 2002 - 2024 por Digifort. Todos direitos reservados.
No exemplo acima, vemos uma pessoa saindo de um prédio, através de uma câmera de corredor e, ao
clicar no link para a câmera externa, esta será carregada, permitindo a investigação avançada do
incidente.
Somente links para câmeras e mosaicos serão exibidos durante a Reprodução de Mídia. Links para
Eventos, Ações, ou diferentes tipos de objetos como Mapas, não serão exibidos.
No Reprodutor de Mídia, você poderá trabalhar da mesma maneira que ao vivo. Diferentes tipos de ações
para a execução de links são suportados, como Substituir o objeto corrente em tela , Adicionar o
objeto em um espaço vazio e Arrastar e soltar um link para um espaço em tela . A ação de abrir
popup não estará disponível no Reprodutor de Mídia.
Você ainda poderá utilizar os links de objetos durante uma Exportação de Sequência , o que torna
este processo ainda mais fácil e intuitivo.
6.9 Trabalhando com Lentes Fisheye e Panamórficas
Se a câmera possuir uma lente 360 Fisheye ou Panomórfica, o sistema poderá exibir a imagem
ajustada, com ajuste de deformidade, permitindo a navegação nesta câmera como se ela fosse uma
câmera PTZ.
Para lentes Fisheye, o sistema integra um Plugin para diversos fabricantes, porém nem todos os
fabricantes são suportados, e as funções de dewarping variam de fabriante para fabricante. O Plugin de
Fisheye também deverá ser instalado na estação de monitoramento.
Para lentes Panomórficas, o sistema possui uma biblioteca embutida para fazer a navegação na
imagem destas câmeras, com interface única, sem necessidade de instalação de plugins adicionais.
O sistema permite o dewarping de câmeras 360 ao vivo e durante a reprodução de mídia.
Ao colocar uma câmera com lente panamórfica na tela, os seguinte botões estarão disponíveis:
Permite navegar na câmera com uma imagem em dewarp. A imagem será exibida como uma
câmera normal. Esta é a principal opção para trabalhar com uma lente Fisheye ou Panomórfica.
90
90 91
123
94 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Divide a imagem em 4 câmeras sem distorção como ilustrado abaixo:
Para operar o PTZ em uma das divisões, basta clicar no número desejado representado pelos ícones:
Gera uma imagem panorâmica como ilustrado abaixo:
Ao deselecionar as opções de Dewarping anteriores, a imagem original da câmera será exibida:
Câmeras 95
© 2002 - 2024 por Digifort. Todos direitos reservados.
Este botão irá esconder ou exibir os botões de controle de dewarping.
6.9.1 Operações de PTZ
O sistema permite o uso de todos os controles de PTZ, como Controles Visuais, Joystick Visual,
Joystick Físico e Zoom em Área em imagens com dewarping 360, oferecendo uma poderosa solução
para navegar nestas imagens.
Exemplo de uso de Joystick Visual para controle de Câmeras 360:
Exemplo de Zoom em Área para controle de Câmeras 360:
Para aprender sobre os diferentes tipos de Controles PTZ, consulte o tópico PTZ 72 .
C
h
a
p
t
e
r
V
I
I
Reprodução de Mídia 97
© 2002 - 2024 por Digifort. Todos direitos reservados.
7 Reprodução de Mídia
O sistema possui um poderoso reprodutor de mídia, com diversos recursos para auxiliar na investigação
de vídeos gravados. Neste capítudo você aprenderá tudo sobre como utilizar todos os recursos do
Reprodutor de Mídia.
7.1 Interface do Reprodutor de Mídia
O Reprodutor de Mídia foi projetado para possuir uma interface simples e intuitiva, onde operadores com
um mínimo de treinamento poderão operar o sistema de forma fácil e eficiente. A interface do reprodutor
é composta por diversos elementos e ferramentas. Veja abaixo seus principais elementos:
1. Seleção de Painel de Controles para PTZ e Audio: Este painel permite abrir os controles para
movimentação de câmeras 360 e controle de áudio.
2. Painel de Seleção de Ferramentas: Permite selecionar diversas ferramentas para a reprodução
de vídeo.
a. Seleção de Horário: Abre o painel com controles para a seleção de horário de reprodução de
mídia.
b. Seleção de Câmeras: Abre o painel com controles para seleção de câmeras e layouts de tela.
c. Linha de Tempo: Abre o painel com controles de linha de tempo.
d. Pesquisa de Movimento: Abre o painel com controles para realizar pesquisa de movimento.
e. Miniaturas: Abre o painel com controles para exibir miniaturas do vídeo sendo reproduzido.
f. Exportação de Mídia: Abre o painel com controles para exportação de vídeo.
g. Exportação de Sequência: Abre o painel com controles para realizar uma exportação de
sequência.
h. Impressão: Abre o painel com controles para impressão.
i. Filtro de Imagens: Abre o painel com controles para aplicar filtro de imagem nas câmeras.
3. Painel de Visualização de Câmeras: Este é o painel onde as câmeras serão exibidas, e este é o
mesmo controle utilizado na interface de vídeo ao vivo. Para maiores detalhes sobre este controle,
consulte o tópico Painel de Visualização de Câmeras e Objetos .
4. Painel de Controle de Reprodução: Este painel fornece ferramentas para o controle da
reprodução de vídeo, como Play, Pause, Avançar, Retroceder, dentre outros.
18
98 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
5. Painel de Controles da Ferramenta Selecionada: Esta sessão da interface é dedicada para os
paineis de ferramenta. Um painel de ferramenta será aberto para cada opção selecionada no Painel
de Seleção de Ferramentas, descrito no Ítem 2.
O Painel de Seleção de Ferramentas do Reprodutor de Mídia é estruturado de forma a seguir uma ordem
lógica para a reprodução de vídeo, começando pela seleção de horário, passando para a seleção de
câmeras e finalmente abrindo a linha de tempo. Esta é a sequência básica para iniciar a reprodução de
vídeo.
7.2 Reproduzindo Vídeos
Para iniciar a reprodução de vídeo, primeiramente abra o Reprodutor de Mídia, através do Menu
Principal do sistema:
O sistema também permite o acesso rápido ao reprodutor de vídeo, a partir de Menus de Contexto de
objetos e na lista de objetos do Cliente de Monitoramento. Veja o tópico sobre Reprodução de vídeo
rápida para aprender mais sobre esta ferramenta. Quando o Reprodutor de Mídia é aberto através do
método de Reprodução Rápida, a seleção de horário e câmeras já estarão preenchidas e o vídeo já
estará em modo de Reprodução.
Se você abriu o reprodutor de vídeo diretamente do Menu Principal, siga os passos dos tópicos a seguir,
Seleção de Horário e Seleção de Câmeras , para iniciar a reprodução, se o reprodutor de vídeo foi
aberto através da Reprodução Rápida, ele já será apresentado em estado de Reprodução e com a barra
de Linha de Tempo aberta.
7.2.1 Seleção de Horário
O primeiro passo para iniciar a reprodução de vídeo, é a seleção de horário. Esta barra de ferramenta,
acessível através do botão Seleção de Horário na barra lateral esquerda, irá fornecer as ferramentas
necessárias para selecionar o horário para iniciar a Sessão de Mídia.
16
131
98 99
Reprodução de Mídia 99
© 2002 - 2024 por Digifort. Todos direitos reservados.
· Intervalo para Reprodução de Vídeo: O sistema fornece algumas opções pré-cadastradas para
facilitar a abertura da sessão de reprodução de vídeo. Você poderá selecionar dentre as opções de
horário pré-definidas, como por exemplo, 30 segundos atrás, 1 hora atrás, 1 dia atrás, dentre outras
opções, ou também poderá escolher a opção Personalizado, onde você deverá especificar a data e
horário inicial e final para a abertura da sessão de mídia.
· Data e Hora Personalizadas: Quando a opção Personalizado estiver selecionado no Intervalo para
Reprodução de Vídeo, os controles para selecionar data e hora iniciais e finais serão abertos.
o Data Inicial: Selecione a data inicial da sessão.
o Data Final: Selecione a data final da sessão.
o Restringir o vídeo à intervalo de horas: Selecione esta opção se deseja especificar a hora inicial
e final. Caso esta opção não esteja selecionada, o sistema irá abrir o vídeo exibindo o conteúdo
completo do intervalo dos dias selecionados.
§ Hora Inicial: Digite a hora inicial. Este é o valor de hora referente à Data Inicial.
§ Hora Final: Digita a hora final. Este é o valor de hora referente à Data Final.
· Próximo: Ao finalizar a seleção de data e hora, clique neste botão para o próximo passo, onde será
exibido a ferramenta de seleção de câmeras.
7.2.2 Seleção de Câmeras
Ao selecionar a opção Seleção de Câmeras, o sistema irá exibir a barra de ferramentas de seleção de
câmeras e mosaicos, conforme exibido acima.
A barra de ferramenta possui controles similares aos controles de mosaicos e layouts para câmeras ao
vivo:
1. Controle de seleção de tipo de layout.
2. Botão para recarregar as câmeras em tela (Apenas disponível quando um mosaico estiver
selecionado).
100 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
3. Botão para adicionar gravações locais. Para maiores informações sobre como reproduzir gravações
locais, veja o tópico sobre Reprodução de Vídeo Local .
4. Controle de seleção de câmeras e mosaicos.
5. Botão para adicionar a câmera selecionada (No controle de seleção de câmeras e mosaicos) em
tela. Este botão irá apenas funcionar para câmeras, e não estará disponível quando um mosaico
estiver selecionado.
6. Botão para remover todas as câmeras da tela
Primeiramente selecione o layout de tela desejado, através do controle de seleção de layout, de acordo
com a quantidade de câmeras que deseja reproduzir. Se você desejar reproduzir câmeras a partir de um
mosaico já gravado, você poderá pular esta etapa.
Após a seleção do layout, você deverá adicionar as câmeras em tela. A fim de fornecer maior
flexibilidade e agilidade na operação do sistema, existem diversas formas de adicionar câmeras em tela
no reprodutor de vídeo. Veja os métodos suportados nos próximos sub-tópicos.
Quando todas as câmeras desejadas estiverem em tela, clique no botão Play para iniciar a Reprodução
de Mídia:
Atenção
Uma câmera pode ser adicionada apenas uma vez em tela, ou seja, você não poderá adicionar a mesma
câmera repetidas vezes em tela, assim como é possível no modo ao vivo.
Dica
Você poderá adicionar, remover ou substituir câmeras em tela a qualquer momento da reprodução de
mídia, sem a necessidade de pausar ou parar o vídeo.
7.2.2.1 A partir da lista de câmeras do reprodutor
A forma mais básica de selecionar câmeras para reprodução é a partir da lista de câmeras disponível
dentro do Reprodutor de Mídia.
Clique sobre o controle de seleção de câmeras e mosaicos, e uma tela de seleção de objetos será
exibida:
135
Reprodução de Mídia 101
© 2002 - 2024 por Digifort. Todos direitos reservados.
Localize as câmeras ou mosaico que deseja adicionar e:
· Clique duas vezes sobre o ícone da câmera ou mosaico.
o Câmera: Ao clicar duas vezes sobre uma câmera, ela será adicionada no próximo espaço vazio na
tela
o Mosaico: Ao clicar duas vezes sobre um mosaico, ele será carregado por completo na tela
· Clique apenas uma vez sobre o ícone do objeto desejado, selecionando-o, e clique OK.
o Câmera: Ao selecionar um objeto de câmera na lista, ela será exibida no controle de seleção de
câmeras. Você também deverá apertar o botão de Adicionar em tela, para adicionar esta câmera
na tela
o Mosaico: Ao selecionar um mosaico na lista e clicar em OK, a tela será fechada e o mosaico será
carregado automaticamente.
7.2.2.2 A partir da lista principal de objetos
Você também adicionar câmeras em tela a partir da lista principal de objetos do sistema, bastando
arrastar e soltar o ícone do objeto desejado para o reprodutor de vídeo. Para aprender mais sobre a lista
de objetos, consulte o tópico Adicionando Objetos em Tela 56 da Lista de Objetos 56 .
102 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Você poderá arrastar e soltar não apenas câmeras e mosaicos a partir da lista principal de objetos do
sistema, mas também outros tipos de objetos que possuem uma câmera como âncora, como por
exemplo Configuração de Analítico e Configuração de LPR, e neste caso, a câmera associada com
o objeto será exibida em tela.
7.2.2.3 A partir de objetos ao vivo
Você poderá arrastar objetos que estão sendo exibidos no modo Ao Vivo, diretamente para o Reprodutor
de Mídia. Para isto, basta arrastar o objeto desejado, a partir da tela principal de monitoramento, para o
local desejado no Reprodutor de Mídia.
Você poderá arrastar objetos como Câmeras ou objetos que possuem uma câmera como âncora, como
por exemplo Configuração de Analítico e Configuração de LPR, e neste caso, a câmera associada
com o objeto será exibida em tela.
7.2.3 Câmeras
Ao adicionar uma câmera em tela, o controle de câmera será exibido:
Este controle de visualização é similar ao controle de visualização de câmeras ao vivo, porém, com
recursos limitados para a reprodução de vídeo.
No cabeçalho da câmera será fornecido as seguintes informações:
· Nome e/ou Descrição da câmera.
· Horário de Reprodução.
· Frames por Segundo gravados (Referente ao segundo atualmente sendo exibido).
· Resolução de imagem.
Reprodução de Mídia 103
© 2002 - 2024 por Digifort. Todos direitos reservados.
· Número do frame de vídeo atual / Número total de frames de vídeo.
· Tipo de compressão de vídeo.
· Marca d’água de autenticidade da imagem: Quando a imagem vinda da câmera é gravada no
disco, um código de segurança é gerado baseado nesta imagem. Se por algum motivo a imagem for
alterada, o código de autenticidade é quebrado, exibindo o valor Invalido em tela. O reprodutor de
vídeo não permitirá a exportação de um frame de vídeo que estiver com sua marca d'agua inválida.
Este objeto possui um menu de contexto simples, acessível com o clique do botão direito do mouse
sobre a imagem:
· Salvar Imagem: Salva a imagem atual em disco. Ao selecionar esta opção, uma janela de diálogo
será exibida, onde você deverá selecionar o nome do arquivo de destino e o seu formato. O sistema
permite salvar a imagem nos formatos: JPG, BMP, PNG, WMF, GIF, TIF.
É possível a execução do Zoom Digital nas imagens das câmeras em reprodução. O controle para Zoom
Digital é idêntico ao modo ao vivo. Para aprender a operar o Zoom Digital das câmeras em reprodução,
consulte o tópico sobre Movimentação através do Zoom Digital .
7.2.4 Linha de Tempo
Ao iniciar a reprodução de mídia, o controle de Linha de Tempo será exibido automaticamente. Você
também poderá acessar a Linha de Tempo a partir do seu botão correspondente na Barra de
Seleção de Ferramentas, no lado esquerdo da interface.
79
104 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Este é um dos principais controles do Reprodutor de Mídia, e fornece a visualização da linha de tempo
de gravação, exibindo o horário de gravação para cada tipo de faixa (Vídeo, Áudio, Metadados), assim
como linhas com informação de movimento e bookmarks. Você poderá utilizar este controle para
navegar em todo o período de gravação.
Veja nos próximos sub-tópicos, as funcionalidades deste controle.
7.2.4.1 Interface
A interface do controle de linha de tempo é divida em 4 painels:
1. Data e Hora corrente da reprodução de mídia.
2. Painel de divisão de horário da linha de tempo: Neste painel é exibido a data e hora
correspondente às linhas de tempo. A divisão de hora pode ser alterada com a função de zoom da
linha de tempo.
3. Painel de câmeras: Neste painel é exibido uma linha para cada câmera no Reprodutor de Mídia.
Nele você poderá selecionar câmeras (A seleção de câmeras é utilizada para recursos como
exportação e pesquisa de movimento) e também será exibido as barras de progresso de pesquisa de
linha do tempo e pesquisa de movimento.
a. Barra de progresso azul: Representa o progresso para finalizar a pesquisa da linha de tempo.
b. Barra de progresso vermelha: Representa o progresso da Pesquisa de Movimento .
4. Painel de linha do tempo: Neste painel é exibido as informações de linha de tempo para cada
faixa de gravação:
a. Linha verde: Gravação de vídeo.
b. Linha laranja: Gravação de áudio.
c. Linha roxa: Gravação de metadados.
d. Linha amarela: Gravação de evento. A gravação de metadados de movimento / evento deve
estar ativada na câmera.
e. Linha movimento: Esta linha é definida pela intensidade do movimento detectado e irá variar em
tons de vermelho. Onde o vermelho mais escuro indica um maior movimento e o mais claro um
movimento menor. A gravação de metadados de movimento / evento deve estar ativada na
câmera.
f. Linha pesquisa de movimento: Esta linha irá conter um gráfico de intensidade de movimento,
produzida pelo recurso de Pesquisa de Movimento .
g. Bookmarks: Bookmarks serão exibidos em uma linha indicando o horário do bookmark e o título
do bookmark.
7.2.4.2 Mover
Para mover a linha de tempo, basta clicar com o botão esquerdo do mouse e arrastar. Ao soltar a linha
de tempo ela irá se mover com inércia.
Durante a movimentação da linha de tempo, todas as imagens das câmeras serão atualizadas
periodicamente, porém, este efeito pode causar lentidão para reprodução de vídeo em conexões lentas.
Você poderá desativar este recurso através do menu de contexto, acessado com o botão direito do
mouse, deselecionando a opção Atualizar instantâneamente ao mover:
109
109
Reprodução de Mídia 105
© 2002 - 2024 por Digifort. Todos direitos reservados.
Ao desativar esta opção, as imagens das câmeras serão apenas atualizadas para o horário da linha de
tempo ao soltar o botão do mouse, e a linha de tempo também não irá mais continuar a mover com
inércia.
7.2.4.3 Zoom
A linha de tempo permite realizar o zoom, para aumentar ou diminuir a resoluçao de tempo exibida.
· Para aumentar a resolução de tempo (Mais zoom), pressione a tecla +
· Para diminuir a resolução de tempo (Menos zoom), pressione a tecla -
Você também poderá aumentar ou diminuir o zoom através do menu de contexto, com o botão direito do
mouse:
A imagem abaixo representa um exemplo de mais zoom na linha de tempo:
A imagem abaixo representa um exemplo de menos zoom na linha de tempo:
7.2.4.4 Seleção de Horário
A linha de tempo permite fazer uma seleção de horário, que pode ser utilizada para exportar vídeos ou
realizar pesquisa de movimento.
Por padrão, assim que o sistema terminar de consultar a linha de tempo, o seu período completo será
selecionado automaticamente.
106 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Para selecionar um período de tempo, clique com o botão direito do mouse sobre a linha de tempo, e
arraste o mouse, mantendo o botão direito pressionado. Uma barra azul de seleção de horário será
exibida no topo da linha de tempo:
Você também poderá selecionar ou deselecionar o horário através do menu de contexto, acessível com
o clique do botão direito do mouse sobre a linha de tempo:
· Selecionar Horário: Abre uma caixa de diálogo para seleção manual do horário, onde você poderá
escolher precisamente o horário desejado:
· Limpar Seleção: Limpa a seleção corrente de horário.
7.2.4.5 Bookmarks
A linha de tempo irá exibir os bookmarks disponíveis para as câmeras em reprodução.
Um bookmark é identificado na linha de tempo com uma barra sobre as linhas de trilhas de mídia,
exibindo o título e a cor do bookmark:
Os bookmarks cuja data e hora iniciais e finais são iguais, serão exibidos apenas com a marca de hora
inicial, e o seu tamanho em tela será determinado pelo tamanho do texto do seu título:
Os bookmarks com data e hora iniciais e finais diferentes, serão exibidos com a marca inicial e final de
acordo com o seu horário, e o seu tamanho em tela será determinado pelo seu horário:
Reprodução de Mídia 107
© 2002 - 2024 por Digifort. Todos direitos reservados.
O menu de contexto, acessível através do clique com o botão direito do mouse sobre a linha de tempo,
oferece algumas opções para trabalhar com Bookmarks:
· Criar Bookmark: Abre a tela de criação de bookmarks para as câmeras em tela. Para aprender
como criar bookmarks, consulte o tópico sobre Bookmarks .
· Avançar para o próximo bookmark: Ao clicar nesta opção, a linha de tempo será avançada até o
próximo bookmark.
· Voltar para o bookmark anterior: Ao clicar nesta opção, a linha de tempo será retrocedida para o
bookmark anterior.
7.2.5 Controles de Reprodução
Este painel permite o controle da sessão de reprodução de mídia.
Exibe data, hora e velocidade atual da reprodução de vídeo.
Pausa a reprodução de vídeo.
Inicia a reprodução de vídeo para trás.
Inicia a reprodução de vídeo para frente.
Para a reprodução de vídeo e fecha a sessão de mídia. Ao iniciar a reprodução novamente (Com
o botão de Play para Frente), a sessão de mídia e linha de tempo será recarregada.
143
108 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Controla a velocidade de reprodução de vídeo.
7.2.5.1 Controle de Avanço e Retrocesso
Este controle permite o avanço e retrocesso das imagens de acordo com o método selecionado. O
método padrão é por tempo, e o tempo de 1 minuto será exibido por padão. No exemplo acima, ao
clicar nos botões com as setas para frente ou para trás, o sistema irá mover o vídeo 1 minuto para frente
ou 1 minuto para trás.
Clique com o botão direito do mouse sobre o centro do controle para mudar o método de avanço ou
retrocesso:
· Horário: Selecione a opção de horário (Padrão) para permitir que ao clicar nas setas direcionais para
frente ou para trás, o sistema avançe ou retroceda o vídeo pelo tempo selecionado. O valor de 1
minuto será exibido por padrão. Para alterar este valor, clique com o botão esquerdo do mouse sobre
o texto e arraste para frente para aumentar os minutos ou para trás para diminuir. Você poderá
escolher a precisão de segundos, para isso, o valor de 1 minuto deve estar sendo exibido, e neste
momento você deve clicar e arrastar para trás, ativando a opção de precisão por segundos (Abaixo de
1 minuto). Se o controle estiver em segundos, você poderá avançar até 1 minuto, e neste momento
você deverá soltar o botão do mouse e clicar novamente se desejar avançar para mais minutos.
· Frame a Frame: Ao selecionar este método, o sistema avançará ou retrocederá o vídeo em 1 frame
ao clicar nos botões direcionais correspondentes. Idealmente o vídeo deve estar pausado para este
método ter efetividade. No caso de reprodução com múltiplas câmeras, ao avançar ou retroceder, o
sistema irá mover 1 frame da câmera cujo frame estiver mais próximo do horário atual de reprodução.
· Bookmark: Ao selecionar este método, o sistema avançará ou retrocederá o vídeo, pulando para o
bookmark mais próximo ou para o bookmark anterior, de acordo com o botão direcional clicado.
7.2.6 PTZ para Câmeras 360
É possível utilizar comandos PTZ para navegar nas imagens gravadas de câmeras com lentes
panomórficas ou fisheye 360. Para abrir o controle basta clicar na opção de joystick como na figura
abaixo:
Reprodução de Mídia 109
© 2002 - 2024 por Digifort. Todos direitos reservados.
· Câmera com lente padrão: O PTZ será utilizado no modo Zoom Digital.
· Câmera com lente 360: As opções apresentadas no capítulo Trabalhando com Lentes Fisheye e
Panamórficas estarão disponíveis na tela e podem ser aplicadas na reprodução de vídeo, assim
como funciona no modo de vídeo ao vivo.
7.2.7 Controle de Áudio
O Áudio é gravado juntamente com o vídeo da câmera, se habilitado. Para escolher a câmera que se
deseja ouvir basta seleciona-la em tela, ou clicar na opção áudio como mostrado na figura abaixo:
Nesse controle é possível selecionar a câmera, ativar a opção mute e visualizar o volume do áudio
gravado.
Se a opção Auto-Selecionar estiver ativada, ao selecionar uma câmera no reprodutor de vídeo, a
câmera será selecionada automaticamente no controle de seleção de câmeras para receber audio.
· Botão Mute: Silencia o áudio da câmera.
· Barra de volume: Mostra o volume do áudio recebido, em tempo real.
· Controle de volume: Permite aumentar ou diminuir o volume de reprodução do áudio.
7.3 Pesquisa de Movimento
O sistema permite realizar uma pesquisa de movimento, em áreas selecionáveis nas câmeras, filtrando
o escopo de reprodução de vídeo para exibir apenas os frames que contém movimento. Este recurso
ajuda muito na busca de algum evento, pois reduz o tempo gasto para analisar as imagens gravadas.
93
110 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Para acessar a pesquisa de movimento, durante uma sessão de reprodução de mídia, clique no ícone
Pesquisa de Movimento no Painel de Seleção de Ferramentas:
Ao abrir esta ferramenta, o sistema exibirá o painel de pesquisa de movimento, e agora você poderá
selecionar áreas para detecção de movimento nas câmeras desejadas. O sistema permite realizar a
pesquisa de movimento em múltiplas câmeras simultâneamente.
· Câmeras: Caixa para seleção de câmeras. Selecione as câmeras para pesquisar movimento.
· Horário de Início: Data de inicio para a pesquisa por movimento. A pesquisa precisa que um período
de inicio e fim seja configurado. Esse horário pode vim preenchido de acordo com o a seleção na linha
do tempo (barra azul).
· Horário de Fim: Data de fim para a pesquisa por movimento. A pesquisa precisa que um período de
inicio e fim seja configurado. Esse horário pode vim preenchido de acordo com o a seleção na linha do
tempo (barra azul).
· Sensibilidade: Sensibilidade de reconhecimento de movimento. 80% é o valor ideal para o
reconhecimento de movimentos significantes da imagem. Se desejar alterar este valor movimente a
barra de modo a obter o valor desejado.
· Apenas Key-frames: Pesquisa de movimento apenas em frames chave (Apenas H.263, MPEG-4,
H.264 e H.265). A velocidade de pesquisa pode ser extremamente aumentada utilizando esta opção,
porém a pesquisa pode ficar menos precisa pois o movimento será reconhecido apenas nos frames
chave. É recomendado que a distância dos frames chave gravados não ultrapasse 2 segundos (Ideal 1
segundo).
· Botão Iniciar Pesquisa: Inicia a pesquisa por movimento. Você poderá acompanhar o progresso da
pesquisa na guia Linha do tempo.
· Botão Limpar Dados: Limpa os dados coletados durante a pesquisa. Esses dados são informações
de onde houve movimento no vídeo, enquanto eles não forem limpos a linha do tempo só exibirá as
gravações onde estiver demarcado o gráfico de movimento (Gráfico vermelho).
Para iniciar uma pesquisa, siga os passos a seguir:
1. Selecione as câmeras desejadas na lista de câmeras. Você deverá selecionar a câmera mesmo que
exista apenas 1 câmera em tela. Caso você esteja reproduzindo múltiplas câmeras, você deverá
escolher as câmeras que deseja realizar a pesquisa.
Reprodução de Mídia 111
© 2002 - 2024 por Digifort. Todos direitos reservados.
2. Selecione áreas sensíveis ao movimento nas câmeras. Para isto clique com o botão esquerdo do
mouse, e enquanto segura este botão, arraste o mouse para criar uma área desejada. Para remover
uma área, faça o mesmo processo, porém segurando o botão direito do mouse e desenhe uma área
maior, que englobe as áreas que você deseja remover. Você poderá criar múltiplas áreas de
pesquisa de movimento na câmera.
3. Selecione as datas e horas iniciais e finais para a pesquisa. Você poderá inserir manualmente estes
valores, nos controles visuais de data e hora, ou você também poderá selecionar a data e hora
através da Seleção de Horário da Linha de Tempo.
4. Configure a sensibilidade da detecção de movimento (Padrão 80% é recomendado).
5. Clique em Iniciar Pesquisa.
Você poderá acompanhar a pesquisa pela barra na guia pesquisa de movimento como mostra a figura
abaixo:
Ou na Linha de tempo como mostra figura abaixo:
Após concluída a pesquisa uma mensagem será exibida como mostra figura abaixo:
Com a pesquisa de movimento finalizada, agora o reprodutor de vídeo irá apenas exibir o vídeo dos
momentos onde movimento foi reconhecido:
No exemplo acima, todas as áreas que estão totalmente em branco indica que não foi encontrado
movimento, o reprodutor irá pular estas áeas automaticamente, permitindo que você analise rapidamente
apenas as imagens que tiveram movimento. Um efeito negativo deste recurso pode ocorrer durante a
reprodução de vídeo com pesquisa de movimento de diversas câmeras simultâneamente, onde existe
105
112 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
movimento em uma câmera e não existe movimento em outra câmera para o mesmo horário. Neste
caso, a mensagem padrão "Não existem gravações para este horário" será exibida na câmera onde o
movimento não foi detectado. Se existir um momento em comum, onde não existe movimento em todas
as câmeras, o sistema irá pular a reprodução automaticamente para o próximo bloco com movimento,
independente de qual câmera este movimento foi reconhecido.
Quando a barra de pesquisa de movimento está sendo exbida na linha de tempo, esta funcionará como
um filtro, permitindo a reprodução de vídeo apenas dos trechos onde movimento foi detectado. Se você
desejar tirar esse filtro, selecione as câmeras desejadas e clique no botão Limpar Dados.
7.4 Miniaturas
O Reprodutor de Mídia permite a pesquisa por miniaturas. Este excelente recurso irá exibir uma
miniatura de diferentes horários da gravação, permitindo a localização rápida de uma cena desejada.
Para acessar esta ferramenta, durante uma sessão de reprodução de mídia, clique no ícone Miniaturas
no Painel de Seleção de Ferramentas:
É possível gerar miniaturas baseadas em fatia de tempo onde o sistema irá exibir as miniaturas com
intervalo de tempo fixo, ou por bookmark, onde o sistema irá exibir uma miniatura para cada bookmark
da câmera. O sistema ainda permite a escolha personalizada do intervalo de tempo e o
tamanho/quantidade de miniaturas em tela.
· Escolher a câmera para a pesquisa: Escolha a câmera que deseja visualizar as miniaturas. A
câmera já deve estar aberta na reprodução dentro do horário determinado anteriormente.
· Tipo de busca:
o Tempo: Divide as miniaturas por intervalos definidos de tempo. No exemplo acima temos
miniaturas a cada 1 hora.
o Bookmark: Exibe as miniaturas dos bookmarks da câmera.
· Data Inicial: Selecione a data inicial que as miniaturas deverão ser exibidas. (Apenas disponível no
tipo de busca por Tempo)
Reprodução de Mídia 113
© 2002 - 2024 por Digifort. Todos direitos reservados.
· Horário Inicial: Selecione o horário inicial que as miniaturas deverão ser exibidas. (Apenas disponível
no tipo de busca por Tempo)
· Intervalo: Selecione o intervalo desejado entre cada miniatura. (Apenas disponível no tipo de busca
por Tempo)
o Tempo em minutos: Caso na opção intervalo esteja selecionada a opção Personalizado, é
possível selecionar o intervalo desejado em minutos nessa opção. (Apenas disponível no tipo de
busca por Tempo)
· Tamanho da imagem: Selecione o tamanho de exibição das miniaturas: Grande, Média ou
Pequena.
· Atualizar: Atualiza a tela com as gravações novas.
Utilize as setas direcionais para esquerda ou para a direita para mudar a página de miniaturas.
Ao clicar em uma miniatura, o vídeo será sincronizado com o horário da miniatura para rápida
visualização do evento.
7.5 Exportação de Mídia
A exportação é um dos recursos mais importantes do reprodutor de mídia, através dele é possível salvar
um trecho do vídeo das câmeras selecionadas, em diversos formatos compatíveis com reprodutores de
vídeo padrão, para serem compartilhadas e visualizadas em diversos computadores ou dispositivos.
Para acessar esta ferramenta, durante uma sessão de reprodução de mídia, clique no ícone
Exportação de Mídia no Painel de Seleção de Ferramentas:
7.5.1 Exportando Vídeos
As opções de exportação são mostradas conforme ilustrado na figura abaixo:
114 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
· Câmeras: Selecione as câmeras desejadas para a exportação. Ao selecionar mais de uma câmera,
os vídeos serão exportados simultaneamente e sincronizadamente.
· Horário de Início: Data de inicio para a exportação. A exportação precisa que um período de inicio e
fim seja configurado. Esse horário pode vim preenchido de acordo com o a seleção na linha do tempo
(barra azul).
· Horário de Fim: Data de fim para a exportação. A exportação precisa que um período de inicio e fim
seja configurado. Esse horário pode vim preenchido de acordo com o a seleção na linha do tempo
(barra azul).
· Formato de Exportação: O sistema possui diversos formatos para o vídeo exportado. Escolha o
formato mais adequado para a sua operação:
o Nativo: Este é o formato de vídeo recomendado, pois uma cópia exata do banco de dados do
sistema é exportada. Com esse tipo de exportação o sistema cria automaticamente uma mídia com
um reprodutor de vídeo idêntico ao reprodutor nativo do sistema e com todas as suas
funcionalidades, inclusive a marca d’água de autenticidade do vídeo. Este formato suporta
criptografia e sistema também irá exportar os bookmarks, juntamente com o vídeo. A limitação
deste formato é que você precisará de um computador com sistema operacional Windows para
poder reproduzir o vídeo.
o MP4: Exporta o vídeo no formato MP4, compatível com a maioria dos reprodutores de vídeo e
sistemas operacionais.
o AVI: Exporta o vídeo no formato AVI que pode ser reproduzido em qualquer reprodutor de vídeo que
possua suporte ao Codec utilizado na exportação. Este formato de exportação não é o mais
recomendado, pois haverá compressão nas imagens e o processo de exportação será lento.
o JPEG: Exporta o vídeo (Apenas vídeo) em imagens JPEG independentes. Você poderá utilizar este
formato para fazer um vídeo de Time-Lapse.
o WAV: Exporta o áudio (Apenas áudio) em formato WAV.
· Limitar o tamanho da Mídia: Selecionando esta opção, o sistema automaticamente irá dividir o
vídeo exportado no tamanho especificado neste campo. Durante a exportação, serão criadas diversas
pastas cujos arquivos terão o tamanho máximo especificado. Esta opção é útil caso você queria
salvar em vídeo em alguma mídia removível que possui tamanho limitado.
7.5.1.1 Formato Nativo
Ao iniciar a exportação no formato Nativo, o sistema exibirá uma caixa de diálogo para a escolha da
pasta de exportação:
A pasta padrão selecionada será a pasta configurada nas opções Gerais do Cliente de
Monitoramento. Nesta tela você poderá escolher a pasta desejada ou também criar uma nova pasta.
26
Reprodução de Mídia 115
© 2002 - 2024 por Digifort. Todos direitos reservados.
Selecionada a pasta, clique em OK. O sistema irá criar automaticamente uma subpasta com as datas
iniciais e finais da exportação, e todo o material será gravado nesta subpasta.
Em seguida, o sistema irá apresentar a tela de configurações da exportação no formato Nativo:
· Nome da Empresa: Forneça o nome da empresa, que será exibido na tela de abertura do reprodutor
de vídeo exportado. Um nome padrão pode ser atribuído nas Configurações do Servidor, utilizando o
Cliente de Administração.
· Responsible for Exporting: Digite o nome do operador responsável por exportar este vídeo. Este
nome será exibido na tela de abertura do reprodutor de vídeo exportado. O nome do usuário
autenticado será exibido por padrão.
· Description: Forneça uma descrição para o vídeo, que será exibido na tela de abertura do reprodutor
de vídeo exportado.
· Criptografia: Selecione esta opção para criptografar o vídeo exportado. O sistema irá utilizar
criptografia AES 256 para a exportação.
o Senha: Forneça a senha de criptografia (Esta senha precisará ser fornecida ao abrir o reprodutor de
vídeo exportado).
o Confirmar Senha: Confirme a senha digitada no campo anterior.
· Marca d'água: Adiciona uma marca d'água nas imagens exportadas. A marca d'água será um texto
que será sobreposto sobre a imagem, utilizando as propriedades definidas abaixo.
o Texto: Forneça o texto para a marca d'água.
o Cor: Forneça a cor da fonte do texto.
o Tamanho: Selecione o tamanho da fonte.
o Posição: Selecione a posição na imagem onde o texto de marca d'água será exibido.
Clique OK para iniciar a exportação.
7.5.1.2 MP4
Ao selecionar a exportação no formato MP4, o sistema irá abrir uma tela com opções para exportação
neste formato:
116 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
· Transcodificar vídeo para H.264: O sistema permite realizar a transcodificação de vídeo para H.264
para melhor compabilidade com reprodutores de vídeo externos. A exportação de vídeo em MP4
suporta os formatos H.264 e H.265, porém o formato H.265 pode não ser compatível com a maioria
dos reprodutores de vídeo, e neste caso é recomendado a ativação deste recurso. Se o esta opção
não estiver ativa e o vídeo original exportado já estiver no formato H.264 ou H.265, e não existir marca
d'água para ser aplicada, ou qualquer transformação de vídeo para ser aplicada, o sistema irá exportar
utilizando o vídeo original. Caso o vídeo original esteja gravado em outro formato como JPEG, MPEG-4
ou MxPEG, ou se for necessário adicionar marca d'água na exportação, ou qualquer transformação de
vídeo para ser aplicada, então o sistema irá automaticamente transcodificar para H.264, utilizando as
configurações padrão de transcodificação.
o Qualidade: Selecione a qualidade da compressão. Quanto mais qualidade, maior será o bitrate
utilizado.
o Redimensionar vídeo: Selecione esta opção caso você deseje redimensionar o vídeo, para
diminuir a sua resolução.
§ Largura: Digite a nova largura do vídeo.
§ Altura: Digite a nova altura do vídeo.
§ Guia: Ao invés de fornecer uma resolução manualmente, você poderá escolher dentre algumas
opções pré-cadastradas.
§ Manter proporção: Selecione esta opção para manter a proporção do vídeo original ao aplicar a
nova resolução.
Clique OK. Em seguida o sistema irá exibir a janela para seleção da pasta de exportação:
Reprodução de Mídia 117
© 2002 - 2024 por Digifort. Todos direitos reservados.
A pasta padrão selecionada será a pasta configurada nas opções Gerais do Cliente de
Monitoramento. Nesta tela você poderá escolher a pasta desejada ou também criar uma nova pasta.
Selecionada a pasta, clique em OK. O sistema irá criar automaticamente uma subpasta com as datas
iniciais e finais da exportação, e todo o material será gravado nesta subpasta.
Após a seleção da pasta de exportação, o sistema irá exibir uma nova tela com opções para adicionar
marca d'água no vídeo exportado:
· Marca d'água: Adiciona uma marca d'água nas imagens exportadas. A marca d'água será um texto
que será sobreposto sobre a imagem, utilizando as propriedades definidas abaixo. Ao ativar esta
opção, o sistema irá forçar a transcodificação de vídeo para H.264.
o Texto: Forneça o texto para a marca d'água.
o Cor: Forneça a cor da fonte do texto.
o Tamanho: Selecione o tamanho da fonte.
o Posição: Selecione a posição na imagem onde o texto de marca d'água será exibido.
Clique OK e a exportação será iniciada.
7.5.1.3 AVI
Ao selecionar a exportação no formato AVI, o sistema irá abrir uma tela com opções para exportação
neste formato:
· Redimensionar vídeo: Selecione esta opção caso você deseje redimensionar o vídeo, para diminuir
a sua resolução. Alguns codecs utilizados na exportação em AVI não irão suportar todas as
resoluções de vídeo que o sistema suporta, então você poderá ativar esta opção para selecionar uma
26
118 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
nova resolução de vídeo para garantir maior compatibilidade com os codecs. Em geral, os codecs
aceitam resoluções cujos valores de largura e altura são divisíveis por 8.
o Largura: Digite a nova largura do vídeo.
o Altura: Digite a nova altura do vídeo.
o Guia: Ao invés de fornecer uma resolução manualmente, você poderá escolher dentre algumas
opções pré-cadastradas.
o Manter proporção: Selecione esta opção para manter a proporção do vídeo original ao aplicar a
nova resolução.
· Sincronização de horário: Por padrão, o exportador de mídia AVI utiliza um sincronizador de horário
para manter o vídeo e áudio sincronizados, assim como manter a reprodução de mídia na taxa de
quadros originalmente gravada, porém o uso do sincronizador impossibilita a exportação de vídeo
gravado com menos de 1 frame por segundo. Para exportar um vídeo com menos de 1 frame por
segundo que não possui trilha de áudio você deverá desabilitar o sincronizador de horário. A
exportação de áudio será desativada se o sincronizador de horário for desativado.
o Taxa de Quadros: Ao desativar o sincronizador de horário, você deverá especificar uma taxa de
quadros que será utilizada na reprodução deste AVI.
· Ativar exportação de áudio: Selecione esta opção para permitir a exportação do áudio juntamente
com o vídeo.
Clique OK. Em seguida o sistema irá exibir a janela para seleção da pasta de exportação:
A pasta padrão selecionada será a pasta configurada nas opções Gerais do Cliente de
Monitoramento. Nesta tela você poderá escolher a pasta desejada ou também criar uma nova pasta.
Selecionada a pasta, clique em OK. O sistema irá criar automaticamente uma subpasta com as datas
iniciais e finais da exportação, e todo o material será gravado nesta subpasta.
Após a seleção da pasta de exportação, o sistema irá exibir uma nova tela com opções para adicionar
marca d'água no vídeo exportado:
26
Reprodução de Mídia 119
© 2002 - 2024 por Digifort. Todos direitos reservados.
· Marca d'água: Adiciona uma marca d'água nas imagens exportadas. A marca d'água será um texto
que será sobreposto sobre a imagem, utilizando as propriedades definidas abaixo.
o Texto: Forneça o texto para a marca d'água.
o Cor: Forneça a cor da fonte do texto.
o Tamanho: Selecione o tamanho da fonte.
o Posição: Selecione a posição na imagem onde o texto de marca d'água será exibido.
Clique OK e o sistema agora irá exibir janela padrão do sistema operacional para seleção de codec AVI:
Nesta tela você deverá selecionar o codec desejado (Recomendado x264vfw) e suas configurações. As
configurações de codec não serão contempladas neste manual pois elas são específicas de cada tipo
de codec instalado no PC.
Clique em OK e a exportação será iniciada.
7.5.1.4 JPEG
Ao selecionar a exportação no formato JPEG, o sistema irá abrir uma tela com opções para exportação
neste formato:
Neste formato de exportação, os frames de vídeo serão exportados independentes, cada um em um
arquivo .JPG.
· Qualidade: Selecione a qualidade de compressão do JPEG.
· Redimensionar vídeo: Selecione esta opção caso você deseje redimensionar as imagens, para
diminuir a sua resolução.
o Largura: Digite a nova largura das imagens.
o Altura: Digite a nova altura das imagens.
o Guia: Ao invés de fornecer uma resolução manualmente, você poderá escolher dentre algumas
opções pré-cadastradas.
o Manter proporção: Selecione esta opção para manter a proporção do vídeo original ao aplicar a
nova resolução.
120 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
· Limitar a frequência das imagens exportadas: A opção de limitar a frequência de imagens
exportadas é muito utilizada para criar vídeos do tipo "timelapse" onde temos um frame por hora, ou
mesmo um frame por dia para vídeos muito longos de construções, onde um mesmo local é gravado
por meses e depois é necessário visualizar o vídeo com apenas alguns frames por semana, etc.
o Taxa de Frames: Selecione a taxa de frames.
o Metrica: Selecione a métrica da taxa de frames.
Clique OK. Em seguida o sistema irá exibir a janela para seleção da pasta de exportação:
A pasta padrão selecionada será a pasta configurada nas opções Gerais do Cliente de
Monitoramento. Nesta tela você poderá escolher a pasta desejada ou também criar uma nova pasta.
Selecionada a pasta, clique em OK. O sistema irá criar automaticamente uma subpasta com as datas
iniciais e finais da exportação, e todo o material será gravado nesta subpasta.
Após a seleção da pasta de exportação, o sistema irá exibir uma nova tela com opções para adicionar
marca d'água nas imagens exportadas:
· Marca d'água: Adiciona uma marca d'água nas imagens exportadas. A marca d'água será um texto
que será sobreposto sobre a imagem, utilizando as propriedades definidas abaixo.
o Texto: Forneça o texto para a marca d'água.
o Cor: Forneça a cor da fonte do texto.
o Tamanho: Selecione o tamanho da fonte.
o Posição: Selecione a posição na imagem onde o texto de marca d'água será exibido.
Clique OK e a exportação será iniciada.
7.5.1.5 WAV
Ao iniciar a exportação no formato WAV, o sistema exibirá uma caixa de diálogo para a escolha da
pasta de exportação:
26
Reprodução de Mídia 121
© 2002 - 2024 por Digifort. Todos direitos reservados.
Clique em OK e a exportação será iniciada. Neste formato de exportação, apenas o áudio (se disponível)
será exportado, em arquivos .WAV.
7.5.1.6 Progresso
Durante a exportação, o sistema irá exibir o seu progresso:
Você poderá acompanhar a quantidade de frames exportadas, assim como o tempo restante estimado
para finalizar a exportação. Você poderá parar a exportação a qualquer momento clicando sobre o botão
Parar Exportação. Caso a exportação seja parada durante o seu progresso, os dados já exportados
não serão apagados.
Se a exportação for bem sucedida, a seguinte caixa de diálogo será exibida:
Esta caixa de diálogo irá fornecer a opção de abrir a pasta com os vídeos exportados.
7.5.2 Reproduzindo vídeos exportados em formato Nativo
Ao exportar o vídeo no formato Nativo, o sistema irá copiar o Reprodutor de Vídeo na pasta exportada,
assim como todos os arquivos de biblioteca necessários para executar o reprodutor. Execute o arquivo
Player.exe ou Player64.exe para iniciar o reprodutor:
122 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Ao executar o reprodutor, caso o vídeo tenha sido exportado com criptografia, a janela abaixo será
exibida. Nela você deverá fornecer a senha de criptografia utilizada nas configurações da exportação no
formato nativo . O reprodutor não deixará você prosseguir caso a senha esteja errada.
A tela principal do Reprodutor de Vídeo exportado será exibida:
A tela principal do reprodutor irá as conter informações sobre o vídeo, que foram fornecidas durante a
configurações da exportação no formato nativo .
O reprodutor também oferece algumas opções que podem ser aplicadas:
· Opções de Redimensionamento: Permite configurar o tipo de redimensionamento de imagens para
melhor visualização.
o Manter proporção: Selecione esta opção para utilizar redimensionamento proporcional à resolução
original da imagem gravada. Esta opção previne distorçoes na imagem.
o Preencher toda a área de imagem: Selecione esta opção para esticar a imagem da câmera em
toda a área de visualização. Esta opção poderá criar distorções nas imagens.
o Redimensionamento Bilinear: Quando as imagens das câmeras são redimensionadas, algumas
distorções podem ocorrer, como bordas serrilhadas. Habilitando este recurso as imagens passarão
114
114
Reprodução de Mídia 123
© 2002 - 2024 por Digifort. Todos direitos reservados.
por um filtro que minimiza esta distorção, mantendo a qualidade da imagem mais próxima da
imagem real.
· Video Decoder: Permite escolher opções para o decodificador de vídeo
o Utilizar multi-thread para decodificação via software: O reprodutor permite o uso de multithread para decodificação de vídeo H.264 e H.265. Esta opção pode ser utilizada para acelerar a
decodificação de vídeo no client, especialmente de imagens ultra megapixel. O uso desta opção irá
adicionar ao menos 1 frame de atraso no vídeo, ou seja, a 30 frames por segundo o atraso adicional
será de pelo menos 33ms enquanto a 7 frames por segundo o atraso adicional será de pelo menos
143ms.
o Utilizar decodificação via hardware NVidia: Habilita o uso de GPU NVidia para decodificar os
vídeos:
§ H.264 Decoder: Ativa a decodificação de vídeo H.264 via GPU NVidia.
§ H.265 Decoder: Ativa a decodificação de vídeo H.265 via GPU NVidia.
§ Decodificação Paralela: O sistema permite paralelismo para decodificação de vídeo H.264 e
H.265 via GPU. Esta opção pode ser utilizada para acelerar a decodificação de vídeo no client,
especialmente de imagens ultra megapixel. O uso desta opção irá adicionar ao menos 1 frame de
atraso no vídeo, ou seja, a 30 frames por segundo o atraso adicional será de pelo menos 33ms
enquanto a 7 frames por segundo o atraso adicional será de pelo menos 143ms.
Apenas clique em Iniciar Normal e o reprodutor de vídeo será executado com todas as funcionalidades
apresentadas no capitulo Reproduzindo vídeos , conforme ilustrado na figura abaixo:
7.6 Exportação de Sequência
Exportação de Sequência é uma ferramenta muito útil para exportar um evento que ocorre
progressivamente através de múltiplas câmeras.
Durante a reprodução de vídeo, é possível trocar de câmera com os links de objetos, ou manualmente,
arrastando e soltando objetos da lista de objetos para o reprodutor de vídeo, para, por exemplo, seguir
um suspeito que está se movimentando entre múltiplas câmeras. A exportação de sequência permite a
exportação do vídeo juntamente com as ações de alternar câmeras, ou seja, é possível criar uma
exportação onde o reprodutor de vídeo exportado irá automaticamente mudar as câmeras, seguindo as
ações criadas pelo operador durante a exportação.
98
124 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Durante a exportação de sequência, todas as operações do operador, como adicionar câmeras em tela,
remover câmeras da tela, alterar layout de mosaico ou carregar outro mosaico, serão gravadas
juntamente com o horário do momento da reprodução de vídeo quando estas ações ocorreram. Durante
a reprodução de vídeo desta exportação de sequência, o reprodutor irá replicar as mesmas ações,
efetivamente alterando as câmeras em tela automaticamente.
Para acessar esta ferramenta, durante uma sessão de reprodução de mídia, clique no ícone
Exportação de Sequência no Painel de Seleção de Ferramentas:
· Histórico de Ações: Este painel exibe os histórico de ações do usuário, como adicionar câmeras em
tela, remover câmeras e alterar mosaicos.
· Iniciar: Inicia a gravação das ações do usuário para exportação de sequência. Para este botão estar
ativo, a sessão de vídeo não pode estar no modo Stop, deve estar em Play ou Pause.
· Limitar o tamanho da Mídia: Selecionando esta opção, o sistema automaticamente irá dividir o
vídeo exportado no tamanho especificado neste campo. Durante a exportação, serão criadas diversas
pastas cujos arquivos terão o tamanho máximo especificado. Esta opção é útil caso você queria
salvar em vídeo em alguma mídia removível que possui tamanho limitado.
Para criar uma exportação de sequência, basta posicionar o vídeo no início do período que deseja
exportar e clicar no botão Iniciar. A partir deste momento, todas as ações (de troca de câmeras e
mosaicos) serão gravadas, então agora você pode clicar em Play e trocar as câmeras conforme for
necessário. No final da sessão basta clicar em Parar e o botão Exportar será habilitado, e a
exportação da sequência poderá ser concluída.
Com isto, é possível criar uma sequencia exportada, onde o Reprodutor de Vídeo irá trocar
automaticamente entre as câmeras, seguindo o momento exato em que o operador trocou entre as
câmeras durante a sessão de exportação. Veja a seguir um exemplo de como realizar a exportação de
sequência.
Neste exemplo iremos criar uma exportação para seguir uma pessoa suspeita saindo de um prédio,
passando por múltiplas câmeras. Primeiramente, posicionamos o vídeo no momento de início da
exportação e clicamos em Iniciar. A partir desse momento todas as ações do usuário serão gravadas.
Ações iniciais serão adicionadas automaticamente, contendo o layout de mosaico e as câmeras iniciais
em tela:
Reprodução de Mídia 125
© 2002 - 2024 por Digifort. Todos direitos reservados.
Agora clicamos em Play e para seguir o suspeito entre as câmeras, utilizamos links de objetos précadastrados, neste caso da imagem acima, iremos clicar na seta apontada para a esquerda, que está
ligada com uma câmera posicionada no corredor de saída:
Ao clicar na seta para a esquerda, a câmera ligada será carregada, e podemos ver o suspeito cruzando
o corredor para sair do prédio. Então prosseguimos com a reprodução de vídeo e agora clicamos na seta
para cima, que representa uma câmera ligada que está fora do prédio:
126 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
A câmera correspondente será carregada e todas essas ações foram salvas no script da exportação. Ao
final da investigação, clicaremos no botão Parar, como mostra a figura anterior, e em seguida o botão
Exportar será ativado e poderemos iniciar a exportação:
Ao clicar no botão Exportar, o sistema irá utilizar a Exportação em formato Nativo, já explicado no
tópico de Exportação em Formato Nativo .
Após configurar a exportação no formato nativo, o sistema irá iniciar a exportação:
114
Reprodução de Mídia 127
© 2002 - 2024 por Digifort. Todos direitos reservados.
Após o término da exportação, você poderá reproduzir a sequência exportada utilizando os passos
descritos no tópico Reproduzindo vídeos exportados em formato Nativo , sendo a única diferença, ao
abrir a janela prinicpal do Reprodutor de Mídia exportado, você irá selecionar o botão Iniciar Sequência
ao invés de Iniciar Normal:
· Iniciar Sequência: Inicia a reprodução de vídeo utilizando o script de sequência para trocar entre as
câmeras automaticamente, seguindo o script criado pelo operador, durante a exportação.
· Iniciar Normal: Inicia a reprodução com todas as câmeras utilizadas para a reprodução em um
mosaico do tipo Automático.
Nota
Este recurso irá exportar todas as imagens de todas as câmeras que fazem parte da sequência durante
todo o período da sequência.
7.7 Impressão
O sistema permite a impressão de uma ou mais imagens em forma de relatório.
Para acessar esta ferramenta, durante uma sessão de reprodução de mídia, clique no ícone Impressão
no Painel de Seleção de Ferramentas:
121
128 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
· Nome da Empresa: Forneça o nome da empresa, que será exibido no relatório. Um nome padrão
pode ser atribuído nas Configurações do Servidor, utilizando o Cliente de Administração.
· Nome do Operador: Digite o nome do operador responsável pela impressão, que será exibido no
relatório. O nome do usuário autenticado será exibido por padrão.
· Notas do Operador: Digite notas gerais sobre este indidente, que será exibido no relatório.
· Imprimir: Imprime o relatório
· Limpar Notas: Limpa o campo de notas gerais.
Preencha os campos e clique em Imprimir
A tela abaixo permite visualizar, exportar e modificar o logo que sairá juntamente com o relatório.
Clique em OK e uma tela para impressão com os dados será aberta.
Reprodução de Mídia 129
© 2002 - 2024 por Digifort. Todos direitos reservados.
Em caso de reprodução de múltiplas câmeras, se você selecionar uma câmera, o relatório irá conter
apenas a imagem da câmera selecionada, caso não exista nenhuma câmera selecionada, então o
sistema irá gerar um relatório com a imagem de todas as câmeras:
Caso o Zoom Digital esteja aplicado em uma imagem, o relatório será gerado apenas com a imagem do
zoom Digital:
130 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Relatório:
7.7.1 Visualizador de Relatórios
O visualizador de relatórios permite a visualização prévia de um relatório antes da da impressão.
O visualizador consiste de uma barra de ferramentas na parte superior e a área de visualização de
relatório no corpo do controle:
Reprodução de Mídia 131
© 2002 - 2024 por Digifort. Todos direitos reservados.
1. Imprime o relatório.
2. Exibe / Esconde minuaturas das páginas no canto esquerdo.
3. Primeira página.
4. Página anterior.
5. Próxima página.
6. Última página.
7. Localizar texto.
8. Localizar próximo texto.
9. Mais zoom.
10. Menos zoom.
11. Alterar tamanho de visualização do formulário.
7.8 Filtros de Imagem
Este recurso permite o operador mudar a características das imagens das câmeras individualmente.
Para abrir os filtros de imagem vá até o menu esquerdo e clique no ícone Filtros de imagem. A
seguinte tela será aberta:
Para aplicar os filtros, selecione a câmera desejada e configure os filtros desejados.
Para aprender sobre os filtros de imagem consulte o capitulo Como configurar os filtros de imagem .
7.9 Reprodução de Vídeo Rápida
O sistema permite a reprodução rápida de vídeos utilizando o menu de contexto (Acessível através do
clique com o botão direito do mouse) de objetos em tela ou da lista de objetos principal do Cliente de
Monitoramento.
84
132 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Ao acessar o menu de contexto de objetos em tela, ou da lista de objetos, você será apresentado com
as seguintes opções:
· Revisão Instantânea: Esta opção fornece uma maneira rápida de reproduzir um vídeo com um
período de tempo pré-configurado. Esta opção se torna muito útil para ambientes onde é necessária a
rápida visualização de eventos, como por exemplo, em Cassinos onde é necessário a revisão
instantânea de um fato. A configuração do período de reprodução pode ser definida nas Configurações
de Reprodução de Vídeo do Cliente de Monitoramento .
· Reprodução de Mídia (Câmera Única): Esta opção permite a reprodução de vídeo da câmera
selecionada. Apenas 1 câmera será aberta no reprodutor de vídeo. Ao selecionar este menu, um submenu será apresentado. Escolha um horário pré-definido na lista apresentada, ou especifique um
horário manualmente através da opção Personalizado. Essa opção somente estará disponível
quando o menu de contexto for aberto a partir de um objeto ao vivo ou um ícone da lista de objetos
que representa um objeto único (Por exemplo uma câmera).
· Reprodução de Mídia em Borda (Câmera Única): Esta opção permite a reprodução de vídeo em
borda da câmera selecionada. Apenas 1 câmera será aberta no reprodutor de vídeo. Ao selecionar
este menu, um sub-menu será apresentado. Escolha um horário pré-definido na lista apresentada, ou
especifique um horário manualmente através da opção Personalizado. Essa opção somente estará
disponível quando o menu de contexto for aberto a partir de um objeto ao vivo ou um ícone da lista de
objetos que representa um objeto único (Por exemplo uma câmera) e a câmera suportar o recurso de
Gravação em Borda.
· Reprodução de Mídia (Múltiplas Câmeras): Esta opção permite a reprodução de vídeo de múltiplas
câmeras simultâneas. Ao selecionar este menu, um sub-menu será apresentado. Escolha um horário
pré-definido na lista apresentada, ou especifique um horário manualmente através da opção
Personalizado. Essa opção somente estará disponível quando o menu de contexto for aberto a partir
de um objeto ao vivo ou um ícone da lista de objetos que representa um grupo de objetos ou um
mosaico. Se o menu de contexto for aberto a partir de um objeto ao vivo, o sistema irá reproduzir o
vídeo de todas as câmeras que estão ocupando a mesma tela que o objeto selecionado.
· Reprodução de Mídia em Borda (Múltiplas Câmeras): Esta opção permite a reprodução de vídeo
em borda de múltiplas câmeras simultâneas. Ao selecionar este menu, um sub-menu será
apresentado. Escolha um horário pré-definido na lista apresentada, ou especifique um horário
manualmente através da opção Personalizado. Essa opção somente estará disponível quando o
menu de contexto for aberto a partir de um objeto ao vivo ou um ícone da lista de objetos que
representa um representa um grupo de objetos ou um mosaico e todas as câmeras suportarem
Gravação em Borda. Se o menu de contexto for aberto a partir de um objeto ao vivo, o sistema irá
reproduzir o vídeo de todas as câmeras que estão ocupando a mesma tela que o objeto selecionado.
35
Reprodução de Mídia 133
© 2002 - 2024 por Digifort. Todos direitos reservados.
O menu de contexto de reprodução rápida de vídeo também será apresentado ao clicar com o botão
direito em objetos que são derivados de câmeras, como por exemplo as Configurações de LPR e
Configurações de Analítico.
Ao selecionar a opção Personalizado, em um sub-menu de pesquisa rápida, a seguinte tela será
apresentada:
· Data Inicial: Selecione a data inicial da sessão.
· Data Final: Selecione a data final da sessão.
· Restringir o vídeo à intervalo de horas: Selecione esta opção se deseja especificar a hora inicial e
final. Caso esta opção não esteja selecionada, o sistema irá abrir o vídeo exibindo o conteúdo
completo do intervalo dos dias selecionados.
o Hora Inicial: Digite a hora inicial. Este é o valor de hora referente à Data Inicial.
o Hora Final: Digita a hora final. Este é o valor de hora referente à Data Final.
7.10 Reprodução de Vídeo Arquivado
Os vídeos arquivados são considerados armazenamento frio e fazem parte do sistema de
arquivamento, que pode ser configurado para copiar todas as gravações do dia para uma pasta de
arquivamento.
Para aprender como ativar o recurso de arquivamento de imagens veja o Manual do Cliente de
Administração.
Devido ao formato de armazenamento das gravações arquivadas, somente é possível reproduzir 1 dia de
arquivamento por sessão de reprodução.
Para reproduzir as gravações arquivadas basta clicar com o botão direito sobre a imagem de uma
câmera ou ítem na lista de objetos, acessando o seu menu de contexto, selecionar a opção
Reprodução de Mídia, e no sub-menu, selecione a opção Gravações Arquivadas. O sistema
também permite a reprodução de múltiplas câmeras arquivadas. Para reproduzir múltiplas câmeras,
selecione a opção Reprodução de Mídia (Múltiplas câmeras) no menu de contexto dos objetos em
tela. Você também poderá reproduzir múltiplas câmeras utilizando o menu de contexto da lista de
objetos, nos grupos de câmeras ou mosaicos, caso os objetos deste grupo ou mosaico suportem
reprodução de vídeo em borda.
134 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Ao selecionar o ítem de Gravações Arquivadas, o sistema irá abrir a tela para seleção do dia para
reprodução:
O sistema irá exibir cores diferentes para os dias no calendário:
· Branco: Não existem gravações arquivadas para este dia, em nenhuma câmera selecionada.
· Verde: Existem gravações arquivadas para este dia, em todas as câmeras selecionadas.
· Laranja: Existe gravação arquivada para este dia para uma ou mais câmeras selecionadas, porém
não existe gravação para todas as câmeras selecionadas.
Selecione o dia desejado e clique OK para abrir o reprodutor de vídeo.
Consulte o tópico Reproduzindo Vídeos para aprender a utilizar o Reprodutor de Mídia.
7.11 Reprodução de Vídeo em Borda
A função de Reprodução de Video em Borda permite a reprodução de vídeo baixando diretamente
das gravações em borda do dispositivo.
Este recurso expande o sistema de Edge Recording (Que permite baixar gravações armazenadas nas
câmeras), com isso, é possível reproduzir as gravações de qualquer dispositivo que tenha Edge
Recording integrado ao sistema.
Também é possível reproduzir vídeo gravado em DVRs/NVRs suportados, possibilitando assim o uso de
gravação distribuída, uma vez que é possível visualizar as gravações diretamente dos equipamentos.
98
Reprodução de Mídia 135
© 2002 - 2024 por Digifort. Todos direitos reservados.
Para reproduzir as gravações em borda basta clicar com o botão direito sobre a imagem de uma câmera
ou ítem na lista de objetos, acessando o seu menu de contexto, e se os dispositivos suportarem Edge
Recording selecionar a opção Reprodução de mídia em borda, e o sub-menu de opções de
reprodução será exibido. O sistema também permite a reprodução de múltiplas câmeras em borda,
possibilitando assim a reprodução de diversas câmeras de um DVR/NVR simultaneamente. Para
reproduzir múltiplas câmeras, selecione a opção Reprodução de mídia em borda (Múltiplas
câmeras) no menu de contexto dos objetos em tela. Você também poderá reproduzir múltiplas câmeras
utilizando o menu de contexto da lista de objetos, nos grupos de câmeras ou mosaicos, caso os
objetos deste grupo ou mosaico suportem reprodução de vídeo em borda.
A reprodução de vídeo em borda será transparente ao usuário, porém terá algumas limitações:
· O sistema irá baixar linearmente as gravações do dispositivo durante a sessão de reprodução de
vídeo, e a linha de tempo será atualizada progressivamente a medida que novas gravações são
temporariamente baixadas.
· O usuário só poderá navegar e visualizar os vídeos já baixados, e não poderá avançar para uma parte
do vídeo que ainda não foi baixada.
· O recurso de Exportação de Sequência não estará disponível para reprodução em borda.
Consulte o tópico Reproduzindo Vídeos para aprender a utilizar o Reprodutor de Mídia.
7.12 Reprodução de Vídeo Local
O Reprodutor de Mídia é capaz de reproduzir vídeos gravados no formato nativo, diretamente da pasta de
gravação. Estes vídeos podem ser de origem de arquivamento, exportação de vídeo ou até mesmo da
gravação local de emergência feita no Cliente de Monitoramento.
Para realizar a reprodução de um vídeo local, você deverá seguir os passos descritos no tópico
Reproduzindo Vídeos , com a única diferença sendo que na seleção das câmeras, ao invés de
selecionar uma câmera através do controle de seleção de câmeras, ou arrastando ícones da lista de
objetos, você irá adicionar um diretório contendo as gravações que deseja reproduzir, para isso, clique
no botão de adicionar gravação local, conforme exemplificado a seguir:
A tela de seleção de diretório será exibida:
98
87
98
136 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Clique no botão de seleção de diretório, representado por um ícone amarelo de pasta e a seguinte tela
será exibida:
Nesta tela de seleção de diretório, você deverá selecionar o diretório raiz, que possui os subdiretórios
Dados e Indices.
Clique OK e o diretório será preenchido na tela anterior:
Uma câmera será adicionada em tela, e as sua gravação será reproduzida a partir do diretório
selecionado:
Reprodução de Mídia 137
© 2002 - 2024 por Digifort. Todos direitos reservados.
Você poderá adicionar múltiplos diretórios locais para a reprodução sincronizada.
A partir deste ponto, você poderá utilizar todos os recursos explicados em tópicos anteriores para a
reprodução de vídeo.
C
h
a
p
t
e
r
V
I
I
I
Alarmes 139
© 2002 - 2024 por Digifort. Todos direitos reservados.
8 Alarmes
O Cliente de Monitoramento possui um poderoso sistema de notificação de alarmes que utiliza Popups
para notificar o operador:
1. A barra de cabeçalho irá trazer informações sobre o evento. Nela você terá:
a. Data e Hora local do evento (Horário da estação de monitoramento).
b. Data e Hora do servidor (Horário do servidor).
c. Mensagem descritiva, indicando o tipo do evento ocorrido
d. Mensagem personalizada (Fornecida apenas via API para Eventos Globais).
2. Painel de objetos . Este controle irá exibir as câmeras ou objetos associados com o evento.
3. Mensagem personalizada, criada a partir do Cliente de Administração. Geralmente contém
instruções para o operador. Este painel irá ser auto-redimensionado, de acordo com o tamanho do
texto.
4. Painel onde o operador pode escrever uma resposta ao evento, que será armazenada no banco de
dados de eventos.
5. Painel de controles.
a. Silenciar: Silencia o som do alarme.
b. Reproduzir: Reproduz o vídeo do evento
c. Fechar: Fecha a tela de alarme
O sistema possui uma grande flexibilidade na configuração das ações (No Cliente de Administração), e
dependendo das ações selecionadas para o evento, alguns paineis não serão exibidos no popup de
alarme. Por exemplo, um alarme pode conter apenas uma mensagem descritiva, sem câmeras ou painel
de resposta do operador:
As janelas de alarme, por padrão, não irão abrir novamente caso o mesmo evento ocorra novamente
enquanto a janela já estiver aberta, porém, quando o evento estiver configurado para requisitar
confirmação por escrito do operador, uma nova janela de alarme será aberta, forçando assim o operador
a confirmar por escrito todos os eventos.
18
140 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Se o alarme possuir uma ação de tocar som, o sistema irá reproduzir o som do alarme e também irá
exibir um ícone, no canto esquerdo do Painel de Controles, para silenciar o som de alarme em
andamento. Este ícone estará apenas disponível enquanto algum som de alarme estiver tocando.
O sistema permite uma extensa configuração do comportamento das janelas de alarme, assim como
organização automática das janelas no monitor, fechamento automático de janelas, posicionamento
personalizado, cor da borda das janelas, dentre outras. Para aprender sobre todas as opções de
personalização da experiência de alarmes, consulte o tópico sobre Configuração de Alarmes do
Cliente de Monitoramento.
8.1 Observações
Quando o painel de observações do operador estiver disponível, você poderá digitar uma resposta ao
evento, fornecendo detalhes, por exemplo, das ações tomadas para validar o evento. O administrador
pode configurar este campo para ser obrigatório, e neste caso, a janela não poderá ser fechada até que
um texto seja adicionado.
O texto fornecido aqui será armazenado no log de eventos , e o sistema ainda fornece um relatório
específico contendo todas as repostas de operadores à eventos. Consulte o tópico de log de eventos
para maiores informações.
8.2 Reprodução
A janela de alarme possui um botão para reproduzir o evento chamado Reproduzir. Ao clicar neste
botão, o sistema irá abrir o Reprodutor de Vïdeo , posicionando a linha de tempo no momento que o
evento ocorreu, e a sessão de mídia será aberta para revisão de até 1 hora anterior e posterior ao horário
de disparo do evento.
Caso um mesmo alarme seja disparado, e o seu popup for reutilizado para o novo disparo, a janela irá
armazenar o horário de todos os alarmes repetidos que ocorreram enquanto ela estava aberta,
permitindo escolher um horário específico quando clicar no botão Reproduzir, como mostra a imagem a
seguir:
8.3 Lista de alarmes locais
A lista de alarmes locais é um registro de quais alertas já foram tratados pelo operador nessa estação
de monitoramento, facilitando o gerenciamento de ambientes com muitos alertas. Esta é uma lista
apenas de alarmes disparados localmente e não incluem todos os eventos ocorridos no servidor. Um
37
250
250
97
Alarmes 141
© 2002 - 2024 por Digifort. Todos direitos reservados.
evento é considerado um alarme quando ele possui ações que abrem um popup na estação de
monitoramento.
Para exibir a lista de alarmes locais, clique no ícone correspondente, no menu principal de opções do
sistema:
Ao abrir a lista, ela aparecerá acima da barra de ferramentas, conforme a imagem abaixo:
A lista contém as seguintes informações:
· Horário: Data que o evento foi recebido.
· Tipo de evento: Qual tipo de evento foi recebido (Analítico, LPR, Evento Global, Detecção de
Movimento, etc).
· Nome do evento: Nome do evento recebido. Este nome estará disponível apenas para eventos
cadastrados dentro de objetos, como Eventos Manuais ou Eventos de Timer.
· Descrição do evento: Descrição do evento recebido.
· Tipo de objeto: Tipo de objeto que disparou o evento.
· Nome do objeto: Nome do objeto que disparou o evento.
· Descrição do objeto: Descrição do objeto que disparou o evento.
· Servidor: Servidor de origem do evento.
· Observações: Observações do operador ao evento, caso a ação de solicitar confirmação tenha sido
enviada.
· Status: Status atual do evento.
o Aberto: O alarme é considerado aberto quando o popup ainda não foi fechado, ou se ele já foi
fechado porém o operador ainda precisa fornecer as observações de tratativa.
o Fechado: O alarme já foi fechado e tratado pelo operador.
Ao receber um novo evento o sistema irá demarcar na lista o evento que ainda está como aberto,
aguardando tratativa:
A marcação permanecerá como aberto até que o operador feche a notificação. As cores para eventos
abertos e fechados podem ser configuradas na tela de configuração de alarmes .
Você poderá abrir o popup de um alarme que já foi fechado através do duplo-clique com o botão
esquerdo do mouse.
Para esconder ou exibir colunas, clique com o botão direito do mouse sobre o título de uma coluna e
selecione a opção Selecionar Colunas.
37
C
h
a
p
t
e
r
I
X
Bookmark 143
© 2002 - 2024 por Digifort. Todos direitos reservados.
9 Bookmark
O recurso de Bookmark permite que marcações sejam feitas na gravação do vídeo. Essas marcações
são feitas com palavras-chave e cores que podem ser pesquisados com facilidade para localizar e
identificar um evento na gravação com facilidade.
A imagem abaixo mostra um Bookmark vermelho indicando um evento na gravação:
9.1 Criando bookmarks
Para criar um Bookmark durante a visualização ao vivo, utilize o controle de bookmarks da barra de
ferramentas:
O primeiro botão com o caractere "+" permite criar um Bookmark a partir de uma data especificada pelo
operador. Ao clicar a seguinte tela estará disponível:
144 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Na parte esquerda da tela podemos escolher as câmeras que desejamos criar o Bookmark. Por padrão
o sistema colocará na lista as câmeras que estiverem no mosaico.
Para adicionar outra câmera, basta clicar em Adicionar câmeras e escolher a câmera desejada.
Para deletar uma câmera, basta selecionar uma ou mais e clicar em Deletar Cameras.
Na parte direita da tela colocaremos as informações relativas ao Bookmark:
· Titulo: Insira o Título do bookmark
· Cor: Selecione a cor do Bookmark
· Data Inicial: Data e horário inicial do evento. O Bookmark será marcado inicialmente nesse horário
· Data Final: O Bookmark poderá ter uma marcação de inicio e uma de fim. Insira a data e horário do
evento nesse campo. Perceba que ao mudar a data final, automaticamente a Duração do evento em
baixo se modificará. OBS: Caso a data final seja a mesma que a inicial, será criado apenas um
Bookmark pontual, ou seja, apenas com a marcação inicial.
· Duração: Configuração da duração do Bookmark. Essa configuração irá mudar a Data final do evento
automáticamente.
· Observações: Digite as observações relacionadas ao evento que será marcado com o Bookmark para
posterior pesquisa.
· Proteger gravações contra deleção: Selecione esta opção para proteger as gravações do período
deste bookmark. Para aprender mais sobre este recurso, consulte o tópico sobre Proteção de
Gravações .
o Adicionar data de expiração: Selecione esta opção para que a proteção destas gravações expire
em um dia configurado.
§ Data: Selecione a data de expiração da proteção.
Depois de preencher as informações clique em OK para criar o Bookmark. A imagem abaixo mostra um
exemplo de bookmark criado, sendo exibido durante a reprodução de vídeo:
272
Bookmark 145
© 2002 - 2024 por Digifort. Todos direitos reservados.
Perceba que esse bookmark possuí dois pontos de marcação que estão sinalizando um bookmark com
intervalo. Bookmarks pontuais, onde a data final é igual a data inicial, serão exibidos apenas com a
marca inicial:
Outra maneira de criar Bookmark é clicando no botão Começar Bookmark, representado pelo circulo
vermelho. Clicando no botão o sistema irá começar contar o tempo de duração do Bookmark até o botão
Finalizar Bookmark, representado pelo quadrado azul, seja apertado:
Quando for finalizado, a tela de configuração do Bookmark se abrirá com a data inicial e final
preenchidas de acordo com o tempo de inicio e fim desse processo:
9.2 Pesquisa de Bookmark
Para pesquisar os Bookmarks criados, basta clicar no botão representado pela lupa:
146 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Clique na lupa para abrir a tela de pesquisa de Bookmarks:
Nessa tela podemos pesquisar todos os Bookmarks criados através de filtros que o sistema
disponibiliza.
Ao selecionar um bookmark é possível trazer a reprodução de vídeo do horário do bookmark, clicando no
botão Vídeo.
Para adicionar filtros clique em Gerenciar Filtros como mostra a imagem abaixo:
Bookmark 147
© 2002 - 2024 por Digifort. Todos direitos reservados.
9.2.1 Filtro de Data
O filtro de data permite pesquisar os Bookmarks pela data selecionada.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Data.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
Escolha intervalo de tempo para pesquisar os Bookmarks criados. Clique em OK e depois na tela
principal de pesquisa clique em Pesquisar:
148 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
9.2.2 Filtro de Palavra-Chave
O filtro de Palavra-chave permite pesquisar os Bookmarks por palavras em seu título ou no campo
observações.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Palavra-Chave.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
· Campo de Pesquisa: Digite a palavra-chave a ser procurada nos títulos dos Bookmarks
· Exatamente: Força o sistema a procurar exatamente o que foi digitado. Caso não marcado, poderá
haver outras palavras junto com a palavra pesquisada.
· Procurar nas observações (pode ser mais lento): O sistema também irá procurar no campo
observações do Bookmark, porém esta pesquisa poderá demandar mais do servidor pois o campo de
observações não é indexado no banco de dados.
Preencha a palavra chave desejada e clique em OK. Na tela principal de pesquisa clique em
Pesquisar:
9.2.3 Filtro de Cor
O filtro de cor permite pesquisar os Bookmarks pelas cores em que foram cadastrados.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Cor.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
Bookmark 149
© 2002 - 2024 por Digifort. Todos direitos reservados.
Selecione as cores desejadas, arrastando da lista esquerda para a lista da direita, e clique em OK. Na
tela principal de pesquisa clique em Pesquisar:
9.2.4 Filtro de Câmeras
O filtro de câmera permite pesquisar os Bookmarks de determinadas câmeras em que foram gravados.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Câmeras.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
150 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Selecione as câmeras desejadas, arrastando da lista esquerda para a lista da direita, e clique em OK.
Na tela principal de pesquisa clique em Pesquisar:
9.2.5 Filtro de Servidor
O filtro de servidor permite filtrar os Bookmarks salvos em determinados servidores onde foram gravados.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Servidores.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
Bookmark 151
© 2002 - 2024 por Digifort. Todos direitos reservados.
Selecione os servidores desejados, arrastando da lista esquerda para a lista da direita, e clique em OK.
Na tela principal de pesquisa clique em Pesquisar:
9.2.6 Mesclando Filtros
Você poderá ativar múltiplos filtros simultaneamente, bastando ativar os filtros desejados na tela de
gerenciamento de filtros. Cada filtro limitado irá limitar o escopo da pesquisa.
Os filtros ativados são mostrados na barra superior onde você pode adicioná-los ou excluí-los conforme
sua necessidade :
152 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Os filtros que forem selecionados se interceptam, isto é, serão filtradas somente as informações que
são comuns a eles.
9.2.7 Gerando Relatórios de Bookmark
Clicando no botão de Imprimir é possível gerar um relatório para impressão com todos os bookmarks:
Selecione o tipo de agrupamento de registro. Você poderá agrupar os registros por diferentes opções.
Selecione a mais apropriada para o seu relatório e clique em OK.
Agora selecione o formato e se você deseja visualizar apenas, imprimir ou exportar (*.pdf, or *..html) e
clique em OK e o visualizador padrão de relatórios será exibido:
O relatório gerado será parecido com a imagem abaixo:
Bookmark 153
© 2002 - 2024 por Digifort. Todos direitos reservados.
9.2.8 Apagando Bookmarks
Para apagar um bookmark, selecione o bookmark desejado e clique com o botão direito do mouse,
selecionando a opção Apagar:
Uma mensagem de confirmação será exibida:
A tela de confirmação também fornecerá uma opção para apagar a proteção de gravação associada com
este bookmark, caso uma exista.
Clique Sim para apagar o bookmark.
O operador precisa ter direitos de apagar bookmarks para poder executar esta função.
C
h
a
p
t
e
r
X
Eventos Globais 155
© 2002 - 2024 por Digifort. Todos direitos reservados.
10 Eventos Globais
O sistema possibilita a criação de Eventos Globais que poderão ser acionados pelo usuário através do
Cliente de Monitoramento. Eventos Globais são eventos que não estão associados com um objeto,
como por exemplo os Eventos Manuais que são associados com uma câmera, os Eventos Globais
são eventos independentes, que quando disparados, geram ações, como por exemplo, acionar sirenes,
abrir portas eletrônicas, acender lâmpadas. Para aprender a configurar os Eventos Globais consulte o
Manual do Cliente de Administração.
Para acionar eventos globais clique sobre o botão Disparar Eventos, localizado na tela principal do
Cliente de Monitoramento, conforme ilustrado na figura abaixo:
A tela de seleção de eventos será exibida:
Nesta tela, você poderá filtrar os eventos utilizando o controle de filtro no topo da tela. O filtro será
aplicado para o nome ou descrição dos eventos.
Para acionar um evento global basta dar um duplo clique no item desejado.
157
C
h
a
p
t
e
r
X
I
Eventos Manuais 157
© 2002 - 2024 por Digifort. Todos direitos reservados.
11 Eventos Manuais
O sistema possibilita a criação de Eventos Manuais que poderão ser acionados pelo usuário através do
Cliente de Monitoramento. Diferentemente dos Eventos Globais , os Eventos Manuais são
associados com uma câmera e podem ser disparados através do menu principal de disparo de eventos
ou através do menu de contexto da câmera, fornecendo assim maior organização do sistema. Assim
como os Eventos Globais, os Eventos Manuais também podem ser utilizados para gerar ações, como
por exemplo, acionar sirenes, abrir portas eletrônicas, acender lâmpadas. Para aprender a configurar os
Eventos Manuais consulte o Manual do Cliente de Administração.
Para acionar eventos manuais, selecione a câmera desejada em tela e clique sobre o botão Disparar
Eventos, localizado na tela principal do Cliente de Monitoramento, conforme ilustrado na figura abaixo:
A tela de seleção de eventos será exibida:
Nesta tela, você poderá filtrar os eventos utilizando o controle de filtro no topo da tela. O filtro será
aplicado para o nome ou descrição dos eventos.
Para acionar um evento manual basta dar um duplo clique no item desejado.
Uma outra forma (Mais rápida e prática) de disparar Eventos Manuais é através do menu de contexto da
câmera. Para isso, clique com o botão direito sobre a câmera em tela, localize o menu Eventos
Manuais (Apenas exibido se a câmera tiver ao menos 1 Evento Manual cadastrado), dentro deste menu
você irá encontrar os eventos manuais cadastrados nesta câmera. Para acionar basta clicar sobre o
ítem desejado:
155
158 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
C
h
a
p
t
e
r
X
I
I
160 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
12 Matriz Virtual
A matriz virtual permite ao operador enviar ou arrastar (drag and drop) qualquer objeto do sistema como
câmeras, mapas, mosaicos, páginas web, analíticos, LPR, para qualquer monitor em que o o Cliente de
Monitoramento esteja aberto em qualquer computador da rede ou para Vídeo Wall. Você pode utilizar a
matriz virtual para criar Video Walls, e também pode utilizá-la para envio de objetos entre operadores do
sistema.
Veja a seguir um diagrama da arquitetura da Matriz Virtual
Nessa imagens temos 3 clientes conectados no servidor, que possui a lista de todos monitores
disponíveis para a matriz virtual, que por sua vez envia a todos os clientes. Dessa maneira todos os
clientes conectados a esse servidor possuirão a lista atualizada de todos os monitores disponíveis, e
todos os clientes poderão interagir com todos os monitores.
A configuração da Matriz Virtual no Cliente é simples, bastando ativar o recurso, e fornecer um nome
único para os monitores da estação, para serem compartilhados com os outros clientes. Consulte o
tópico sobre Configuração da Matriz Virtual para maiores detalhes.
Após configurar os monitores do cliente para estarem disponíveis na Matriz Virtual, certfique-se que o
usuário logado no Cliente de Monitoramento possui direto de juntar-se à Matriz Virtual (para saber como
conceder direitos de usuário consulte o Manual do Cliente de Administração).
12.1 Utilizando a Matriz virtual
Existem diversas maneiras de enviar objetos para outros monitores da Matriz Virtual. O usuário logado
no Cliente de Monitoramento deverá ter direitos de operar a Matriz Virtual para poder enviar objetos. Veja
a seguir as os diferentes métodos de operação.
46
Matriz Virtual 161
© 2002 - 2024 por Digifort. Todos direitos reservados.
12.1.1 Enviando Objetos únicos através do Menu de Contexto
A forma mais fácil de enviar objetos para a Matriz Virtual é através do Menu de Contexto dos objetos ou
da lista de objetos. Você poderá enviar qualquer tipo de objeto visual para ser exibido na matriz, assim
como mosaicos completos .
Clique com o botão direito do mouse sobre um objeto visual em tela, como por exemplo uma câmera, ou
um mapa e utilize a opção Matrix Virtual - Exibir em, do menu:
Ao selecionar esta opção, um submenu com todos os monitores disponíveis será exibido, clique no
monitor desejado para enviar o objeto.
É possivel realizar o mesmo procedimento descrito acima na lista de objetos, como demostra a figura
abaixo:
Caso você esteja enviando um objeto único, e o monitor de destino possuir um layout de múltiplas
posições carregado, o sistema irá abrir uma tela para permitir você a selecionar qual a posição deseja
enviar o objeto:
163
162 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Nesta tela, no quadrante esquerdo, o sistema irá carregar o layout que está sendo exibido no monitor de
destino, e você poderá selecionar uma posição do layout para exibir o objeto, ou você poderá escolher a
opção Tela Cheia, e o layout do monitor será substituído pelo layout de 1 objeto e este objeto será
exibido em tela cheia.
Se o layout de 1 objeto já estiver carregado no monitor de destino, e você estiver enviando um objeto
único, então esta tela não será exibida e o objeto será enviado instantâneamente.
12.1.2 Enviando Objetos únicos através da função Arrastar-e-Soltar
Você poderá enviar objetos que estão sendo exibidos ao vivo na tela, utilizando a função de arrastar-esoltar. Para isso, primeiramente selecione a Lista de Monitores na barra lateral de controles, e em
seguida, clique com o botão esquerdo do mouse sobre o objeto que deseja enviar, e segurando o botão
apertado, arraste o objeto e solte sobre o monitor desejado, na lista de monitores:
Caso você esteja enviando um objeto único, e o monitor de destino possuir um layout de múltiplas
posições carregado, o sistema irá abrir uma tela para permitir você a selecionar qual a posição deseja
enviar o objeto:
Matriz Virtual 163
© 2002 - 2024 por Digifort. Todos direitos reservados.
Nesta tela, no quadrante esquerdo, o sistema irá carregar o layout que está sendo exibido no monitor de
destino, e você poderá selecionar uma posição do layout para exibir o objeto, ou você poderá escolher a
opção Tela Cheia, e o layout do monitor será substituído pelo layout de 1 objeto e este objeto será
exibido em tela cheia.
Se o layout de 1 objeto já estiver carregado no monitor de destino, e você estiver enviando um objeto
único, então esta tela não será exibida e o objeto será enviado instantâneamente.
12.1.2.1 Enviando Objetos de Mapas
Se você estiver visualizando um Mapa Sinóptico , você poderá arrastar objetos de dentro do mapa,
diretamente para a lista de monitores, e o objeto selecionado de dentro do mapa será enviado para a
matriz virtual, ao invés do próprio mapa. Se você arrastar o mapa em sí, então o objeto de mapa será
exibido na Matriz Virtual.
12.1.3 Enviando Múltiplos Objetos
O sistema permite enviar múltiplos objetos em tela, grupos de objetos, ou mosaicos prontos,
diretamente para a Matriz Virtual.
12.1.3.1 Enviando Mosaicos
Para enviar mosaicos salvos, você poderá clicar com o botão direito do mouse sobre o ítem do mosaico
desejado, diretamente na lista de objetos, e selecionar a opção Matriz Virtual - Exibir em:
167
164 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Ao selecionar esta opção, um submenu com todos os monitores disponíveis será exibido, clique no
monitor desejado para enviar o mosaico.
12.1.3.2 Enviando Objetos em Tela
A Matriz Virtual permite o envio de todos os objetos que estão presente em tela, juntamente com suas
configurações (Perfil de mídia, posição de zoom, posição da lente 360, filtro de imagens e detecção de
movimento, etc).
Ao clicar com o botão direto sobre algum objeto (ou na matriz vazia) a opção de Matriz Virtual - Exibir
todos os objetos em será fornecida com a lista dos monitores disponíveis.
Ao selecionar esta opção, um submenu com todos os monitores disponíveis será exibido, clique no
monitor desejado para enviar todos os objetos em tela.
OBS: O mosaico não precisa ser salvo para esta opção ser utilizada.
12.1.3.3 Enviando Grupos de Objetos
Para enviar grupos de objetos, você poderá clicar com o botão direito do mouse sobre o ítem do grupo
desejado, diretamente na lista de objetos. O sistema irá fornecer duas opções:
Matriz Virtual 165
© 2002 - 2024 por Digifort. Todos direitos reservados.
· Matriz Virtual - Exibir em: Selecione esta opção para exibir todos os objetos diretos do grupo para a
Matriz Virtual.
· Matrix Virtual (Incluir subgrupos) - Exibir em: Selecione esta opção para exibir todos os objetos
diretos e todos os objetos de todos os subgrupos do grupo selecionado para a Matriz Virtual.
Ao selecionar esta opção, um submenu com todos os monitores disponíveis será exibido, clique no
monitor desejado para enviar os objetos do grupo selecionado.
12.2 Lista de Monitores
Utilize a lista de monitores para enviar objetos para a matriz virtual via arrastar-e-soltar e para verificar os
objetos que estão sendo exibidos atualmente em todos os monitores. Consulte o subtópico sobre a
Lista de Monitores 60 no tópico da Interface do Cliente de Monitoramento para detalhes sobre esta lista.
C
h
a
p
t
e
r
X
I
I
I
Mapas Sinópticos 167
© 2002 - 2024 por Digifort. Todos direitos reservados.
13 Mapas Sinópticos
O Mapa Sinóptico é uma ferramenta que permite a visualização de um mapa do ambiente, contendo
imagens de plantas baixas, por exemplo, onde é possível posicionar objetos como câmeras, sensores,
alarmes, acionadores, dentre outros, e obter uma visualização geral do ambiente. O Mapa Sinóptico
também pode ser utilizado como um painel de controles de ações e status de eventos e sensores. O
sistema permite grande flexibilidade na confecção dos mapas. Para aprender como criar mapas
sinópticos, consulte o Manual do Cliente de Administração.
Após cadastrar o seu mapa ele estará disponível na lista de objetos do sistema, conforme a figura
abaixo:
Interface do objeto visual de Mapa Sinóptico:
1. Barra de Ferramentas, contendo opções para controle e ajuste do mapa para melhor visualização.
2. Filtro de Objetos: Ao digitar o nome de algum objeto, o mapa irá filtrar e exibir apenas os objetos que
satisfazem o filtro.
3. Barra de Título, contendo o nome e a descrição deste objeto de mapa.
4. Área de visualização do mapa
5. Barra de status de conexão do mapa com o servidor.
168 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
13.1 Barra de Ferramentas
1. Recarrega o mapa
2. Estica a área de visualização do mapa para ocupar toda a área do controle, sem manter a proporção
original do mapa.
3. Estica a área de visualização do mapa para toda a área do controle, mantendo a proporção original
do mapa.
4. Exibe o mapa, sem redimensionamento.
5. Filtro de Redimensionamento: Permite a escolha do filtro de redimensionamento do mapa.
a. Nearest: Filtro de redimensionamento rápido, utiliza pouca CPU, porém produz resultados com
artefatos de redimensionamento, como bordas serrilhadas.
b. Draft: Este filtro de redimensionamento produz um resultado melhor que o Nearest, porém
utilizando um pouco mais de CPU.
c. Linear: Este filtro de redimensionamento produz os melhores resultados de redimensionamento,
porém utilizando mais CPU.
6. Abrir todas as câmeras do mapa em uma janela popup.
7. Controle de zoom para visualização do mapa, disponível apenas quando a opção 4 estiver
selecionada.
Para aumentar a área de visualização do mapa, você poderá fechar a barra de ferramentas através do
ícone representado por uma seta, no centro inferior da barra:
13.2 Status de Objetos
Alguns objetos, como câmeras, exibem o seu status de funcionamento com um ícone ao lado do ícone
do objeto:
Caso o objeto suporte status de funcionamento, os seguintes ícones serão utilizados:
Mapas Sinópticos 169
© 2002 - 2024 por Digifort. Todos direitos reservados.
Objeto em funcionamento. Câmeras com este ícone estão em funcionamento, porém não estão
gravando em disco atualmente.
Objeto em funcionamento. Câmeras com este ícone estão em funcionamento, gravando em disco
atualmente.
Objeto fora de funcionamento
13.3 Janela de Status e Preview de Câmeras
Ao posicionar o mouse em cima do ícone de uma câmera, será exibido um breve relatório sobre a
câmera como mostra figura abaixo.
A caixa de diálogo irá exibir:
· Título com nome e status: O nome da câmera será exibido no título, que terá cor verde caso a
câmera esteja em funcionamento ou vermelho se ela estiver fora de funcionamento.
· Status de ativação: Indica se a câmera está ativada ou desativada.
· Status de funcionamento: Status em texto, indicando se a câmera está funcionando.
· Preview: Fornece uma imagem da câmera atualizada a cada segundo. OBS: O buffer de imagens
para a câmera deve estar ativado nas configurações desta câmera. Consulte o Manual do Cliente de
Administração para aprender a configurar este buffer.
· Campos Restritos: Os seguintes campos estarão disponíveis apenas se o usuário possuir direito de
visualização de status de câmeras.
o Endereço: Endereço IP da câmera.
o Porta: Porta de comunicação.
o Disco Utilizado: Espaço em disco utilizado pelas gravações.
o Dias de Gravação: Total de dias gravados.
o Dias Estimados de Gravação: Estimativa de dias de gravação para o limite atual de disco aplicado
para a câmera.
o Entrada de Alarme: Status das portas de Entrada de IO da câmera.
13.4 Janela de Status de Dispositivos de IO
Ao posicionar o mouse em cima do ícone de um dispositivo de IO, será exibido um breve relatório sobre
o dispositivo como mostra figura abaixo.
170 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
A caixa de diálogo irá exibir:
· Título com nome e status: O nome do dispositivo será exibido no título, que terá cor verde caso o
dispositivo esteja em funcionamento ou vermelho se ele estiver fora de funcionamento.
· Status de ativação: Indica se o dispositivo está ativado ou desativado.
· Status de funcionamento: Status em texto, indicando se o dispositivo está funcionando.
· Campos Restritos: Os seguintes campos estarão disponíveis apenas se o usuário possuir direito de
visualização de status de dispositivos de IO.
o Endereço: Endereço IP do dispositivo.
o Porta: Porta de comunicação.
o Entrada de Alarme: Status das portas de Entrada de IO do dispositivo.
13.5 Abrir Câmeras
É possível abrir câmeras que estão representadas no Mapa Sinóptico através de ícones. Para isto,
clique duas vezes sobre o ícone de uma câmera, e um popup com o controle de câmera será exibido:
Ao clicar em duas vezes em outra câmera, ela será exibida na mesma janela:
Mapas Sinópticos 171
© 2002 - 2024 por Digifort. Todos direitos reservados.
Você poderá abrir a câmera em uma nova janela, para isto, segure a tecla shift e dê um duplo clique no
ícone da câmera. Com múltiplas janelas abertas, ao clicar duas vezes em um outro ícone de câmera,
ela será exibida na janela que estiver com o foco, a menos que a tecla shift esteja pressionada (Para
criar uma nova janela).
Você também poderá arrastar um ícone de câmera diretamente para um espaço na tela de
monitoramento, como mostra a imagem abaixo:
Você também poderá abrir todas as câmeras do mapa, em um popup, clicando no botão na barra
de ferramentas superior do controle.
13.6 Acionadores
Os mapas sinópticos permitem adicionar ícones que, quando acionados, podem disparar um evento, ou
uma ação física, como por exemplo acionar relés e alarmes.
172 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Para ativar um evento ou ação, basta dar um duplo clique no botão correspondente, como no exemplo a
seguir:
Os mapas também permitem adicionar status de eventos, assim como status de portas de entrada de
alarme, permitindo a criação de mapas que podem ser utilizados como uma espécie de painel de
controles:
Outros objetos como Presets para câmeras PTZ também poder ser adicionados no mapa, e podem ser
acionados com o duplo clique.
Consulte o Manual do Cliente de Administração para maiores informações sobre como configurar
eventos, status, ações, presets e acionadores nos mapas.
13.7 Links para Mapas
O sistema de mapa permite adicionar links para outros mapas, criando assim a possibilidade de mapas
multi-nível. No exemplo abaixo temos um ícone em formato de globo, que ao ser clicado duas vezes, irá
abrir o mapa associado:
Mapas Sinópticos 173
© 2002 - 2024 por Digifort. Todos direitos reservados.
Assim como nas câmeras, você poderá arrastar o link do mapa para um espaço no mosaico de
monitoramento, permitindo a abertura do mapa ligado:
Chapter
XIV
Mapas Operacionais 175
© 2002 - 2024 por Digifort. Todos direitos reservados.
14 Mapas Operacionais
Os mapas operacionais possuem avançadas aplicações dentro de servidores com diversas câmeras,
monitorando diversos pontos, por exemplo, em uma cidade.
Este é um recurso que através da integração com o Google Maps, permite a criação de mapas de
navegação e mapas de eventos.
Os mapas de navegação fornecem uma visão geral com o geo-posicionamento de todas as câmeras
do sistema (Que possuem geo-posicionamento ativado) e irá permitir o acesso a estas câmeras através
de ícones referenciados no mapa. Se o Cliente de Monitoramento estiver conectado em múltiplos
servidores, o mapa operacional irá concentrar e exibir os objetos de todos os servidores
automaticamente.
Os mapas de eventos fornecem, em tempo real, a posição do evento (se o mesmo estiver georeferenciado) no mapa, quando o mesmo ocorrer, criando uma poderosa interface de visualização e
navegação que oferece uma visão detalhada dos locais onde estão ocorrendo eventos e permite ao
operador acessar as câmeras próximas a um evento, acelerando assim a resposta ao evento.
Os Mapas Operacionais podem ser encontrados na sua lista de objetos:
Interface do objeto visual de Mapa Operacional:
1. Barra de Ferramentas, contendo opções para controle e ajuste do mapa para melhor visualização.
176 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
2. Barra de Endereço. Exibe o endereço do evento selecionado e permite a navegação para um
endereço especificado.
3. Filtro de Objetos: Ao digitar o nome de algum objeto, o mapa irá filtrar e exibir apenas os objetos que
satisfazem o filtro.
4. Barra de Título, contendo o nome e a descrição deste objeto de mapa.
5. Lista de Eventos.
6. Área de Visualização do Mapa
O mapa operacional, quando exibido, irá conectar-se em todos os servidores conectados no Cliente de
Monitoramento, recebendo informações das câmeras e de eventos de todos eles, possibilitando que o
operador tenha um controle e visualização geral do sistema.
14.1 Barra de Ferramentas
1. Recarrega o mapa
2. Abrir todas as câmeras do mapa em uma janela popup.
3. Mudar entre Modo Dia e Modo Noite
Para aumentar a área de visualização do mapa, você poderá fechar a barra de ferramentas através do
ícone representado por uma seta, no centro inferior da barra:
14.2 Eventos
Uma das maiores utilidades do Mapa Operacional é a sua capacidade de exibir eventos georeferenciados, fornecendo a posição do evento no mapa, assim como informações adicionais sobre o
evento:
Mapas Operacionais 177
© 2002 - 2024 por Digifort. Todos direitos reservados.
Neste exemplo, eventos de detecção de movimento estão sendo recebidos neste mapa e são exibidos
na coluna de registros à esquerda, ao clicar em um dos registros, o mapa irá centralizar no local onde o
evento ocorreu.
Apenas os eventos que possuem latitude e longitude serão exibidos na tela.
A lista de eventos permite apagar um evento específico ou apagar todos os eventos, para isto, basta
clicar com o botão direito sobre a lista de eventos, exibindo o Menu de Contexto, selecionar a opção
desejada:
· Apagar selecionado: Apaga o evento selecionado.
· Apagar todos registros: Limpa a lista de eventos.
Após receber um evento, o operador pode visualizar simultaneamente todos os objetos (Câmeras e
Mapas Sinópticos), que estão sendo exibido no perímetro atual do mapa, bastando clicar no botão de
Exibir Objetos da barra de ferramentas, tornando a tratativa por parte do operador algo muito mais ágil
e com um alcance sem precedentes:
O evento exibido no mapa, também terá todas as variáveis de evento listadas em tela. Estas variáveis
possuem informações extras sobre o evento:
178 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Nota
A lista de eventos irá manter os últimos 50 eventos.
14.3 Abrir Câmeras e Mapas
É possível abrir câmeras e mapas sinópticos que estão representadas no Mapa Operacional através de
ícones. O sistema possui algumas opções para exibição do objeto, que podem ser configuradas nas
Configurações de Mapas Operacionais .
Com a configuração padrao:
· Clicar duas vezes sobre um ícone de câmera, ou mapa operacional, o objeto ligado irá substituir o
objeto de mapa operacional em tela.
· Clicar duas vezes sobre um objeto, segurando a tecla shift, o objeto ligado será adicionado em um
quadrante do mosaico que estiver livre, se nenhum quadrante estiver livre, um popup será exibido com
o objeto.
· Clicar duas vezes sobre um objeto, segurando as teclas ctrl + shift, o objeto ligado será exibido em
uma janela de popup.
49
Mapas Operacionais 179
© 2002 - 2024 por Digifort. Todos direitos reservados.
Você também poderá abrir todos os objetos em exibição no perímetro atual do mapa, em um popup,
clicando no botão na barra de ferramentas superior do controle.
Chapter
XV
Analiticos 181
© 2002 - 2024 por Digifort. Todos direitos reservados.
15 Analiticos
O analítico é um conjunto de ferramentas que processa as imagens das câmeras de uma forma
inteligente. Esse processamento incluí contagem de objetos, controle de fluxo, objetos deixados e
retirados, detecção de face e outros que veremos com detalhe a seguir. Esse sistema é capaz de
detectar o momento em que há infrigência às normas pré definidas e disparar alarmes com o intuito de
atrair a atenção do operador.
Os analíticos podem complementar o monitoramento de diversas formas como disparar alarmes,
arquivar eventos e gerar relatórios.
Os Analíticos podem ser encontrados na sua lista de objetos:
A interface do objeto de analítico é composta por uma câmera e uma lista de eventos.
15.1 Barra de eventos
A lista de eventos é utilizada para exibir em tempo real, os eventos disparados por esta Configuração de
Analítico. A lista sempre estará limpa quando o objeto for adicionado em tela, e irá manter os últimos
100 eventos que ocorreram enquanto o objeto estiver em tela.
182 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
· Data Inicial: Data inicial do evento.
· Data Final: Data final do evento.
· Zona: Zona, se disponível, em que o evento foi disparado.
· Evento: Tipo de evento.
· Classe: Classe de objeto, se disponível, que disparou o evento.
· Regra: Regra, se disponível, que disparou o evento.
A barra de eventos organiza os eventos de modo decrescente, ou seja, o ultimo a ser disparado no topo.
É possível visualizar o vídeo de algum evento clicando sobre ele com o botão direto e depois na opção
Reproduzir vídeo do evento.
15.2 Menu de Contexto
O Menu de Contexto do analítico oferece diversas opções para personalização da exibição dos
metadados de analítico, assim como controles como reset de contadores, reprodução de vídeo,
bookmarks e matriz virtual.
Clique com o botão direito do mouse sobre a câmera do objeto de analítico para acessar o Menu de
Contexto:
Analiticos 183
© 2002 - 2024 por Digifort. Todos direitos reservados.
No menu onde se encontra o ponteiro do mouse estão disponíveis as seguintes funcionalidades:
· Resetar Contadores: Se existirem contadores em tela, a opção Resetar todos os contadores
estará disponível, selecione esta opção para resetar o valor de todos os contadores em tela, ou clique
com o botão direito sobre um contador específico para obter a opção para resetar o valor apenas
daquele contador.
· Mostrar Zonas: Habilita ou desabilita a visualização das zonas configuradas na tela.
· Mostrar Linhas: Habilita ou desabilita a visualização das linhas configuradas na tela.
· Mostrar Contadores: Habilita ou desabilita a visualização dos contadores na tela.
· Mostrar Objetos: Habilita ou desabilita a visualização do retângulo que envolve o objeto na tela.
· Mostrar Rastro do Objeto: Habilita ou desabilita a visualização do rastro do objeto na tela.
· Mostrar Classificação do Pbjeto: Habilita ou desabilita a visualização da classificação do objeto
(carro, pessoa, Sem classificação, etc).
· Mostra Área do Objeto: Habilita ou desabilita a visualização do calculo da área do objeto.
· Mostrar Velocidade do Objeto: Habilita ou desabilita a visualização do calculo de velocidade do
objeto.
· Mostrar Altura do Objeto: Habilita ou desabilita a visualização do cálculo da altura do objeto.
· Mostrar Cores do Objeto: Habilita ou desabilita a visualização das cores do objeto.
· Mostrar os Objetos não Alarmados: Mostrar os objetos que não estão disparando nenhuma regra
de analítico. Um objeto quando dispara um evento do analítico tem seu contorno alterado da cor
amarela para a cor vermelha.
184 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
· Mostrar Características de Reconhecimento de Face: Mostra as características de
reconhecimento de face, como sexo, humor, idade, etc.
· Reprodução de Vídeo: Consulte o tópico sobnre Reprodução Rápida de Vídeo .
· Bookmark: Abre a tela para criar bookmark com esta câmera selecionada.
· Matriz Virtual: Consulte o tópico sobre Enviar Objetos para a Matriz Virtual .
15.3 Gravação e Metadados
É possível ativar a gravação dos dados gerados pelo analítico juntamente com as imagens da câmera.
Para aprender a ativar esse recurso veja o Manual do Cliente de Administração.
Quando uma câmera é utilizada por um analítico e tem a função de gravação de metadados ativa, em
sua gravação é possível visualizar os dados gerados pela análise de vídeo:
Durante a reprodução de vídeo, uma barra de cor roxa será exibida na linha de tempo mostrando a trilha
de gravação dos metadados.
O sistema ainda permite a seleção de quais metadados de analítico serão exibidos, basta clicar com o
botão direito em cima da imagem como mostra a imagem abaixo:
131
143
161
Analiticos 185
© 2002 - 2024 por Digifort. Todos direitos reservados.
OBS: Os metadados também serão incluídos em uma exportação de vídeo no formato nativo.
15.4 Registros de Analíticos
O sistema possui uma poderosa ferramenta para pesquisa e relatórios de registros de analítico. Neste
tópico você aprenderá a pesquisar, gerar relatórios e gráficos de analíticos.
15.4.1 Pesquisando registros
Para realizar a busca de registros clique na opção Pesquisa de Registros como mostra imagem
abaixo:
A seguinte tela será exibida:
186 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Nesta tela é possível pesquisar os registros por diversos métodos. Veremos cada um deles nos
próximos capítulos
Quando a tecla Pesquisar é pressionada, o sistema irá realizar a consulta dos registros de acordo com
os filtros selecionados na barra de filtro:
Todo evento de analítico começa e termina em uma determinada data e hora, portanto é possível ver a
gravação do exato momento clicando no registro desejado e logo de pois no botão Vídeo como na figura
abaixo:
Analiticos 187
© 2002 - 2024 por Digifort. Todos direitos reservados.
Após clicar em vídeo o Reprodutor de Mídia será exibido, trazendo o vídeo do evento.
15.4.1.1 Detalhes do registro
Alguns registros de analíticos possuem anexos como uma foto do momento do evento.
Para ver o detalhe de algum registro basta selecioná-lo e clicar em Detalhes do registro selecionado
como mostra a imagem abaixo:
15.4.1.2 Pesquisando com filtros
A pesquisa permite a seleção de diversos filtros para auxiliar na busca por registros. Clique no botão
Gerenciar Filtros para adicionar novos filtros:
15.4.1.2.1 Filtro de Data
A pesquisa por data permite filtrar os registros pela data selecionada.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Data.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
Você verá duas opções: Data completa e Data fracionada.
97
188 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
A pesquisa por Data Completa irá filtrar os registros encontrados entre a Data / Hora Inicial e Data /
Hora Final:
Já a pesquisa pela Data Fracionada permite uma maior variedades de combinações, fornecendo uma
poderosa ferramenta para criar relatórios personalizados. Selecione o campo Data Fracionada como
mostra a imagem a seguir:
Essa tela possuí as seguintes funcionalidades:
· Dia: Configuração do dia inicial e dia final para filtrar os eventos contidos entre esses dias.
· Mês: Configuração do mês inicial e mês final para filtrar os eventos contidos entre esses meses.
· Ano: Configuração do ano inicial e ano final para filtrar os eventos contidos entre esses anos.
· Semana: Configuração da semana inicial e semana final para filtrar os eventos contidos entre esses
dias.
· Horas: Configuração da hora inicial e hora final para filtrar os eventos contidos entre esses horas.
Essa pesquisa permite mesclar campos e trazer resultados como o exemplo abaixo:
Desejo pesquisar os eventos entre os dias 1 e 20, entre os meses de julho e dezembro, entre os anos
de 2023 e 2024, que se encaixem entre segunda e sexta-feira e nos horários das 06:00:00 até 22:00:00.
Escolha intervalo de tempo para pesquisar os registros. Clique em OK e depois na tela principal de
pesquisa clique em Pesquisar:
Analiticos 189
© 2002 - 2024 por Digifort. Todos direitos reservados.
15.4.1.2.2 Filtro de Câmeras
O filtro de câmera permite pesquisar os registros de determinadas câmeras em que foram gravados.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Câmeras.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
Selecione as câmeras desejadas, arrastando da lista esquerda para a lista da direita, e clique em OK.
Na tela principal de pesquisa clique em Pesquisar:
15.4.1.2.3 Filtro de Classe de Objetos
O filtro de classe de objetos permite pesquisar os registros de eventos que foram disparados por
determinada classe, como por exemplo um carro ou uma pessoa.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Classes de Objetos.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
190 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Selecione as classes desejadas, arrastando da lista esquerda para a lista da direita, e clique em OK.
Na tela principal de pesquisa clique em Pesquisar:
15.4.1.2.4 Filtro de Tipo de Evento
O filtro de tipo de evento permite pesquisar os registros de eventos de determinado tipo, como por
exemplo um Presença, Loitering, etc.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Eventos.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
Analiticos 191
© 2002 - 2024 por Digifort. Todos direitos reservados.
Selecione os tipos de eventos desejados, arrastando da lista esquerda para a lista da direita, e clique
em OK. Na tela principal de pesquisa clique em Pesquisar:
15.4.1.2.5 Filtro de Zonas
O filtro de zonas permite pesquisar os registros de eventos que ocorreram em determinadas zonas
configuradas.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Zonas.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
192 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Selecione as zonas desejadas, arrastando da lista esquerda para a lista da direita, e clique em OK. Na
tela principal de pesquisa clique em Pesquisar:
15.4.1.2.6 Filtro de Regras
O filtro de regras permite pesquisar os registros de eventos que foram disparados por determinadas
regras configuradas.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Regras.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
Analiticos 193
© 2002 - 2024 por Digifort. Todos direitos reservados.
Selecione as regras desejadas, arrastando da lista esquerda para a lista da direita, e clique em OK. Na
tela principal de pesquisa clique em Pesquisar:
15.4.1.2.7 Filtro de Servidores
O filtro de servidores permite pesquisar os registros de eventos que foram disparados por servidores
conectados no Cliente de Monitoramento.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Servidores.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
194 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Selecione os servidores desejados, arrastando da lista esquerda para a lista da direita, e clique em OK.
Na tela principal de pesquisa clique em Pesquisar:
15.4.1.2.8 Mesclando os filtros
Você poderá ativar múltiplos filtros simultaneamente, bastando ativar os filtros desejados na tela de
gerenciamento de filtros. Cada filtro limitado irá limitar o escopo da pesquisa.
Os filtros ativados são mostrados na barra superior onde você pode adicioná-los ou excluí-los conforme
sua necessidade :
Os filtros que forem selecionados se interceptam, isto é, serão filtradas somente as informações que
são comuns a eles.
Analiticos 195
© 2002 - 2024 por Digifort. Todos direitos reservados.
15.4.1.3 Impressão de Registros
Clicando no botão de Imprimir é possível gerar um relatório para impressão com todos os registros
filtrados:
Selecione o tipo de agrupamento de registro. Você poderá agrupar os registros por diferentes opções.
Se você desejar incluir um pequeno snapshot do evento (Se o evento possuir) no relatório, selecione a
opção Exibir Snapshots. Selecione as opções mais apropriadas para o seu relatório e clique em OK.
Agora selecione o formato e se você deseja visualizar apenas, imprimir ou exportar (*.pdf, or *..html) e
clique em OK e o visualizador padrão de relatórios será exibido:
O relatório gerado será parecido com a imagem abaixo:
196 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
15.4.2 Gerando Gráficos
O Gráfico de Analítico é uma poderosa ferramenta que trás instantaneamente gráficos estatísticos de
todos os eventos do sistema. Nos próximos capítulos exploraremos detalhadamente essa ferramenta.
Para iniciar, na tela de registros de analítico clique na aba Gráficos e a seguinte tela será exibida:
Esse capítulo se utilizará dos conceitos de filtros explicados no capítulo Pesquisando com filtros .
Selecione os filtros desejados para a geração do gráfico.
15.4.2.1 Configurações do gráfico
A ferramenta de gráficos permite uma grande flexibilidade na hora de gerar os relatórios.
As configurações dos gráficos tem as seguintes configurações:
187
Analiticos 197
© 2002 - 2024 por Digifort. Todos direitos reservados.
15.4.2.1.1 Tipos de gráficos
Essa opção define o tipo do gráfico que será mostrado. Dentre as opções estão:
· Barras:
· Linhas:
· Pizza:
15.4.2.1.2 Série e Distribuição
Com a combinação das funções Série e Distribuição é possível obter poderosos resultados nos
relatórios.
Na figura a seguir foi configurado o tipo de gráfico como Barras e no campo Série a opção Tipos de
Eventos e selecionamos no filtro os eventos de Presença e Reconhecimento Facial. Dessa maneira
o gráfico mostrará no eixo Y a somatória de todos os eventos do sistema.
A opção Distribuição mostrará os dados em uma determinada amostragem de tempo. Na figura abaixo
essa distribuição está Diária, ou seja, temos a amostragem de todos os dias do mês (1, 2 ,3 ...31):
198 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Para entender como funciona os filtros veja o capítulo Pesquisando com filtros .
A opção Distribuição está relacionada ao eixo X do gráfico mais precisamente com o tempo da
amostragem e possuí as seguintes funcionalidades:
· Horária: Divide as amostragens dos registros em horas (das 00hrs às 23hrs).
· Diária: Divide as amostragens em dias (do dia 1 ao dia 31).
· Semanal: Divide as amostragens em dias da semanas (de Domingo à Sábado).
· Mensal: Divide as amostragens em meses (de Janeiro a Dezembro).
· Anual: Divide as amostragens em anos (anos que contenham registros).
A opção Série está relacionada ao eixo Y do gráfico mais precisamente com as amostragens e possuí
as seguintes funcionalidades:
· Eventos: A opção série de eventos irá exibir no eixo Y, a contagem dos tipos de eventos encontrados
nos registros.
· Classes de objetos: A opção série de Classes de Objetos irá exibir no eixo Y, a contagem dos
objetos de acordo com a sua classe.
187
Analiticos 199
© 2002 - 2024 por Digifort. Todos direitos reservados.
· Zonas: A opção série de Zonas irá exibir no eixo Y, a contagem dos registros por zona de analítico.
· Câmeras: A opção série de Zonas irá exibir no eixo Y, a contagem dos registros por câmeras.
· Regras: A opção série de Regras irá exibir no eixo Y, a contagem dos registros por regras.
· Dias da Semana: A opção série de Dias da Semana irá exibir no eixo Y, a contagem de todos os
eventos por dias da semana. É recomendável utilizar esta opção junto com a Distribuição Mensal ou
Anual.
200 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
· Mêses: A opção série de Mêses irá exibir no eixo Y, a contagem de todos os eventos por mêses do
ano. É recomendável utilizar esta opção junto com a Distribuição Mensal ou Anual.
· Anos: A opção série de Anos irá exibir no eixo Y, a contagem de todos os eventos por anos. É
recomendável utilizar esta opção junto com a Distribuição Anual.
15.4.2.2 Opcões do Gráfico
Existe algumas opções que permitem mudar como o gráfico apresentado para impressão ou melhor
visualização.
Na figura a cima temos as seguintes funcionalidades:
· Título: Acrescenta um título para o gráfico:
Analiticos 201
© 2002 - 2024 por Digifort. Todos direitos reservados.
· Sub-Título: Acrescenta um sub-título para o gráfico:
· Legenda do Eixo X: Acrescenta uma legenda para o eixo X:
202 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
· Legenda do Eixo Y: Acrescenta uma legenda para o eixo Y:
· 3D: Opção que permite tornar os aspecto do gráfico 3D. A imagem abaixo mostra o gráfico em com a
opção habilitada e desabilitada respectivamente:
· Legenda: Habilita ou Desabilita o quadro de legendas no gráfico. A Imagem abaixo mostra o gráfico
com a opção habilitada e desabilitada respectivamente:
Analiticos 203
© 2002 - 2024 por Digifort. Todos direitos reservados.
· Mostrar Valores: Habilita ou Desabilita os valores no gráfico. A Imagem abaixo mostra o gráfico com
a opção habilitada e desabilitada respectivamente:
· Imprimir: O botão Imprimir abre uma tela com o relatório para impressão ou que pode ser salvo no
disco como mostra a figura abaixo:
204 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
15.4.3 Gerando relatórios
Os relatórios analíticos permitem que o usuário possa gerar um relatório personalizado com os registros
de analítico.
É possível visualizar o relatório em tela, imprimir ou exportar o relatório para PDF e CSV.
A tela de relatório é acessivel pelo botão Relatório na parte superior da tela como mostra imagem
abaixo:
O funcionamento segue a mesma lógica da tela de gráficos apresentado no capítulo anterior Gerando
Gráficos . É possível filtrar as informações pelas opções encontradas na barra lateral esquerda e
selecionar o tipo de série e Distribuição desejada. Abaixo segue uma imagem de um relatório onde a
Distribuição é Horária e a Série são Tipos de Eventos:
196
Analiticos 205
© 2002 - 2024 por Digifort. Todos direitos reservados.
Relatório impresso:
15.5 Pesquisa de Metadados
A Pesquisa de Metadados, também conhecida como Pesquisa Forense, permite realizar uma pesquisa
detalhada diretamente nos metadados gravados pelo sistema de analítico. Metadados geralmente
contém características de objetos, como tipo, altura, velocidade, cor, dentre outros.
Para realizar a busca de registros clique na opção Pesquisa de Metadados como mostra imagem
abaixo:
A seguinte tela será exibida:
206 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Para começar a busca primeiro selecione os filtros desejados clicando em Gerenciar Filtros no canto
superior direito. Consulte o tópico de Filtros para aprender a configurar os filtros.
Após selecionar os filtros desejados, clique em Pesquisar para iniciar:
O sistema irá trazer uma lista de objetos reconhecidos, de acordo com os filtros selecionados. Esta
será uma lista de snapshots, com o recorte da imagem do objeto. Você poderá selecionar o tamanho
dos quadrantes utilizando os botões .
207
Analiticos 207
© 2002 - 2024 por Digifort. Todos direitos reservados.
Ao selecionar um registro, você poderá ver o snapshot da sua cena com o objeto no painel superior
direito:
Logo abaixo é apresentado um mini-player para revisão rápida do evento:
O player possui as funções de retroceder, pausar, avançar e repetir. Além disso também temos a opção
de abrir o reprodutor de mídia clicando no botão .
Ao clicar com o botão direito do mouse sobre o Player você terá o seu Menu de Contexto com opções
para Salvar a imagem e Opções para Renderização de Metadados:
Consulte o tópico sobre o Menu de Contexto de Analítico para aprender mais sobre as funções de
rendereização disponíveis neste Menu de Contexto.
15.5.1 Filtros
A pesquisa permite a seleção de diversos filtros para auxiliar na busca por registros. Clique no botão
Gerenciar Filtros para adicionar novos filtros:
98
182
208 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
15.5.1.1 Filtro de Data
O filtro de data permite pesquisar os metadados de analítico pela data selecionada.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Data.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
Selecione a Data e Hora Inicial e Data e Hora Final para a pesquisa.
15.5.1.2 Filtro de Câmeras
O filtro de câmeras permite pesquisar os metadados de analítico para as câmeras selecionadas. Este
filtro é obrigatório.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Câmeras.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
Selecione as câmeras desejadas, arrastando da lista esquerda para a lista da direita.
15.5.1.3 Filtro de Área
O filtro de área permite pesquisar os metadados de analítico, filtrando objetos cuja área atende às
condições deste filtro.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Área.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
· Tipo de Comparação: Selecione o método de comparação de valores
o Igual à: Área do objeto deve ser igual ao valor fornecido.
o Diferente de: Área do objeto deve ser diferente do valor fornecido.
o Menor que: Área do objeto deve ser menor que o valor fornecido.
o Menor ou igual a: Área do objeto deve ser menor ou igual ao valor fornecido.
o Maior que: Área do objeto deve ser maior que o valor fornecido.
o Maior ou igual a: Área do objeto deve ser maior ou igual ao valor fornecido.
Analiticos 209
© 2002 - 2024 por Digifort. Todos direitos reservados.
· Valor: Forneça o valor de referência para a comparação.
· Métrica: Selecione o tipo de métrica
o Metrico: Utiliza o sistema métrico, onde a área é calculada em Metros Quadrados.
o Imperial: Utiliza o sistema imperial, onde a área é calculada em Pés Quadrados.
15.5.1.4 Filtro de Altura
O filtro de altura permite pesquisar os metadados de analítico, filtrando objetos cuja altura atende às
condições deste filtro.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Altura.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
· Tipo de Comparação: Selecione o método de comparação de valores
o Igual à: Altura do objeto deve ser igual ao valor fornecido.
o Diferente de: Altura do objeto deve ser diferente do valor fornecido.
o Menor que: Altura do objeto deve ser menor que o valor fornecido.
o Menor ou igual a: Altura do objeto deve ser menor ou igual ao valor fornecido.
o Maior que: Altura do objeto deve ser maior que o valor fornecido.
o Maior ou igual a: Altura do objeto deve ser maior ou igual ao valor fornecido.
· Valor: Forneça o valor de referência para a comparação.
· Métrica: Selecione o tipo de métrica
o Metrico: Utiliza o sistema métrico, onde a altura é calculada em Metros.
o Imperial: Utiliza o sistema imperial, onde a altura é calculada em Pés.
15.5.1.5 Filtro de Velocidade
O filtro de velocidade permite pesquisar os metadados de analítico, filtrando objetos cuja velocidade
atende às condições deste filtro.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Velocidade.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
· Tipo de Comparação: Selecione o método de comparação de valores
o Igual à: Velocidade do objeto deve ser igual ao valor fornecido.
o Diferente de: Velocidade do objeto deve ser diferente do valor fornecido.
o Menor que: Velocidade do objeto deve ser menor que o valor fornecido.
o Menor ou igual a: Velocidade do objeto deve ser menor ou igual ao valor fornecido.
o Maior que: Velocidade do objeto deve ser maior que o valor fornecido.
o Maior ou igual a: Velocidade do objeto deve ser maior ou igual ao valor fornecido.
· Valor: Forneça o valor de referência para a comparação.
· Métrica: Selecione o tipo de métrica
o Metrico: Utiliza o sistema métrico, onde a velocidade é calculada em KM/H.
o Imperial: Utiliza o sistema imperial, onde a velocidade é calculada em MPH.
15.5.1.6 Filtro de Classificação
O filtro de classificação permite pesquisar os metadados de analítico, filtrando objetos cuja classe
atende às condições deste filtro.
210 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Classificação.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
Selecione as classes de objetos desejadas, arrastando da lista esquerda para a lista da direita.
15.5.1.7 Filtro de Cor
O filtro de classificação permite pesquisar os metadados de analítico, filtrando objetos cujas cores
atende às condições deste filtro.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Cor.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
· Cores: Selecione as cores que deseja filtrar.
· Lógica de Multi-Cor: Caso você selecione mais do que uma cor, escolha qual a lógica será utilizada
para a comparação das cores.
o E: O objeto deve possuir todas as cores selecionadas.
o OU: O objeto deve possuir qualquer uma das cores selecionadas.
· Proporção: Selecione a proporção de cor que o objeto deve possuir.
o Igual à: Proporção de cor do objeto deve ser igual ao valor fornecido.
o Diferente de: Proporção de cor do objeto deve ser diferente do valor fornecido.
o Menor que: Proporção de cor do objeto deve ser menor que o valor fornecido.
o Menor ou igual a: Proporção de cor do objeto deve ser menor ou igual ao valor fornecido.
o Maior que: Proporção de cor do objeto deve ser maior que o valor fornecido.
o Maior ou igual a: Proporção de cor do objeto deve ser maior ou igual ao valor fornecido.
o Valor: Valor de proporção desejado.
Analiticos 211
© 2002 - 2024 por Digifort. Todos direitos reservados.
· Etiquetas: Alguns analíticos fornecem as cores específicas de partes do objeto, como por exemplo a
cor do Torso, ou a cor das Pernas, selecione as etiquetas desejadas, se disponíveis, para por
exemplo pesquisar todas as pessoas que possuem calça vermelha.
15.5.1.8 Filtro de Idade
O filtro de idade permite pesquisar os metadados de analítico, filtrando objetos cuja idade (Geralmente
detectada por sistemas de Reconhecimento de Face) atende às condições deste filtro.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Idade.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
· Tipo de Comparação: Selecione o método de comparação de valores
o Igual à: Idade da pessoa deve ser igual ao valor fornecido.
o Diferente de: Idade da pessoa deve ser diferente do valor fornecido.
o Menor que: Idade da pessoa deve ser menor que o valor fornecido.
o Menor ou igual a: Idade da pessoa deve ser menor ou igual ao valor fornecido.
o Maior que: Idade da pessoa deve ser maior que o valor fornecido.
o Maior ou igual a: Idade da pessoa deve ser maior ou igual ao valor fornecido.
· Valor: Forneça o valor de referência para a comparação.
15.5.1.9 Filtro de Características de Pessoa
O filtro de características de pessoas permite pesquisar os metadados de analítico, filtrando resultados
de pessoas cujas características atendem às condições deste filtro. Estas características geralmente
são geradas por sistemas de Reconhecimento de Face.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Características de
Pessoas.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
Selecione as características desejadas, arrastando da lista esquerda para a lista da direita.
212 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
15.5.1.10 Filtro de Servidores
O filtro de câmeras permite pesquisar os metadados de analítico apenas para as câmeras dos
servidores selecionados.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Câmeras.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
Selecione as câmeras desejadas, arrastando da lista esquerda para a lista da direita.
15.5.2 Avançado
O sistema permite ajustes avançados para otimizar a pesquisa:
· Processamento: Esta opção permite o ajuste entre performance e assertividade, tornando a busca
mais rápida em torca da análise de um número menor de frames.
o Processar todos os frames: Durante a pesquisa, todos os frames de metadados gravados serão
analisados. Esta opção terá maior assertividade na busca, porém irá demorar mais tempo para
finalizar.
o Processar X frames por segundo: Permite analisar apenas uma certa quantidade de frames por
segundo que foram gravadas, aumentando assim a velocidade de busca. O valor padrão de 5 frames
por segundo é recomendado para manter uma assertividade aceitável.
· Tempo para fechar objeto depois de não ser detectado: Quanto tempo o sistema deve
considerar para que um objeto seja considerado como parte do background caso não esteja mais
sendo detectado.
· Distância máxima de objetos entre frames: Qual a distancia (em % do cenário) o sistema deve
considerar para que seja considerado um novo objeto pelo sistema.
· Mapa de Calor: Se esta opção for selecionada o sistema irá gerar um mapa de calor independente
para cada câmera pesquisada.
Analiticos 213
© 2002 - 2024 por Digifort. Todos direitos reservados.
15.5.3 Mapa de Calor
Ao realizar uma busca por metadados o sistema também pode gera um mapa de calor (caso a opção
esteja selecionada nas opções avançadas de filtros). Esse mapa pode ser acessado assim que a busca
é finalizada, clicando na aba Mapa de Calor no canto superior esquerdo:
Ao selecionar a aba o sistema dará então a opção de selecionar a câmera (dentre as selecionadas nos
filtros) e então exibirá um mapa de calor, de acordo com os filtros aplicados. Este mapa de calor será
criado a partir da movimentação dos objetos filtrados:
A legenda do mapa de calor irá indicar a quantidade de objetos que foram reconhecidas, sendo que as
áreas mais vermelhas (quentes) possuem maior movimentação:
O mapa pode ser salvo como imagem ao clicar no botão de salvar no canto inferior direito:
214 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Chapter
XVI
216 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
16 LPR
O LPR é um conjunto de serviços que processa as imagens das câmeras para leitura automática de
placas de veículos. O sistema possui diversas ferramentas para trabalhar com os resultados como
pesquisas, relatórios, alarmes, automação, dentre outros.
As Configurações de LPR podem ser encontrados na sua lista de objetos:
Interface:
1. Lista de placas reconhecidas e botões de ações rápidas.
2. Imagem da captura.
3. Painel de câmeras ao vivo. Este painel possui a câmera principal (Primeira) e todas as câmeras
periféricas.
4. Painel de informações sobre a placa reconhecida.
5. Painel de status de conexão com o servidor.
16.1 Lista e Informações de Placas Reconhecidas
Durante a operação do LPR, as placas reconhecidas serão adicionadas na lista de placas, ao lado
esquerdo do controle. A lista sempre estará limpa quando o objeto for adicionado em tela, e irá manter
os registros dos últimos 100 reconhecimentos que ocorreram enquanto o objeto estiver em tela.
LPR 217
© 2002 - 2024 por Digifort. Todos direitos reservados.
Ao selecionar um registro, você poderá ver a imagem do momento da captura, assim como as
informações sobre a placa:
Caso o engine de LPR utilizado possua o recurso de confiabilidade de leitura por caractere, a placa será
representada com cores de caractere de acordo com a sua confiabilidade:
· Preto: Alto índice de confiabilidade para o caractere.
· Azul: Médio índice de confiabilidade para o caractere.
· Vermelho: Baixo índice de confiabilidade para o caractere.
Por padrão, o primeiro registro de placa reconhecida estará sempre selecionado, e neste caso, sempre
que uma placa nova for reconhecida, ela será exibida automaticamente. Se você selecionar qualquer
outro registro, o sistema irá manter o novo registro selecionado, e novas placas apenas serão
adicionadas na lista, porém a seleção do registro não será alterada, portanto, lembre-se sempre de
voltar a seleção para o primeiro registro, caso você tenha selecionado qualquer outro registro para
análise.
As placas reconhecidas que fazerem parte de alguma lista de LPR, irá exibir uma tag, com o nome da
lista e a cor da lista para fácil identificação. A placa pode ser reconhecida em múltiplas listas, e neste
caso, uma tag para cada lista será apresentada:
218 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
As listas podem ser utilizadas para diversas aplicações, como por exemplo, uma lista negra de veículos
roubados, ou uma lista de moradores que pode ser utilizada para abrir um portão automaticamente.
Consulte o Manual do Cliente de Administração para aprender a criar e associar placas em listas.
O controle inferior irá exibir informações mais completas sobre a placa reconhecida:
Este controle irá exibir o dia e a hora da leitura, a imagem da placa, as tags de listas de placas,
informações sobre o proprietário da placa, informações recebidas através do LPR Bridge e informações
adicionais do engine (se disponível), como país reconhecido, tipo de veículo, cor de veículo, cor da
placa, velocidade do veículo, dentre outras.
16.2 Cadastrando Placas
As placas reconhecidas podem ser adicionadas no cadastro de placas do sistema. O cadastro de
placas é particularmente útil pois você poderá fornecer informações complementares sobre uma placa,
assim como associar com listas de placas para organização e geração de eventos. Para cadastrar uma
placa reconhecida, clique com o botão direito do mouse sobre a placa desejada e selecione a opção
Plate Registration:
Você também pode cadastrar uma placa na lista de placas através do botão Plate Registration, abaixo
da lista de placas.
Uma janela será aberta para cadastro das placas:
LPR 219
© 2002 - 2024 por Digifort. Todos direitos reservados.
Se você acessou a tela através do botão direito do mouse sobre uma placa reconhecida, então o campo
Placa já estará preenchido, caso você tenha acessado através do botão Plate Registration, você
poderá cadastrar qualquer placa que desejar.
· Placa: Digite a placa para ser cadastrada.
· Proprietário: Proprietário do veículo
· Observações: Observações gerais sobre esta placa (Campo livre).
· Listas: Se você desejar adicionar esta placa em alguma lista de placa (Previamente cadastradas no
Cliente de Administração), basta selecionar as listas desejadas.
· Ativar expiração de placa: Selecione esta opção para que a validade desta placa no cadastro
expire automaticamente.
o Data Inicial: Data e hora inicial (A placa estará válida a partir desta data).
o Data de Expiração: Forneça a data de expiração.
16.3 Alterando Placas Reconhecidas
O sistema permite que o operador altere os caracteres de uma placa reconhecida (Caso ele identifique
um erro no reconhecimento do caractere), caso ele tenha direito para isso. Para alterar um registro,
clique com o botão direito do mouse sobre a placa e selecione Alterar Placa Reconhecida:
O sistema irá exibir uma pequena tela, onde você poderá digitar a nova placa:
220 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
16.4 Reproduzindo Vídeo
Para reproduzir o vídeo de um registro de reconhecimento ao vivo, selecione a placa desejada e clique
com o botão direito sobre o registro e selecione a opção Reproduzir Vídeo:
16.5 Registros de LPR
O sistema possui uma poderosa ferramenta para pesquisa e relatórios de registros de LPR. Neste
tópico você aprenderá a pesquisar, gerar relatórios e gráficos de LPR.
16.5.1 Pesquisando Registros
Para realizar a busca de registros clique na opção Registros de LPR (LPR records) como mostra
imagem abaixo:
A seguinte tela será exibida:
Nesta tela é possível pesquisar os registros por diversos métodos. Veremos cada um deles nos
próximos capítulos
LPR 221
© 2002 - 2024 por Digifort. Todos direitos reservados.
Quando a tecla Pesquisar é pressionada, o sistema irá realizar a consulta dos registros de acordo com
os filtros selecionados na barra de filtro:
Para reproduzir o vídeo de um registro, selecione o ítem desejado e clique sobre o botão Vïdeo. O
Reprodutor de Mídia será aberto com o vídeo do momento da passagem do veículo.
16.5.1.1 Pesquisa Rápida
Você poderá abrir a tela de pesquisa rapidamente através do controle visual de LPR ao vivo.
Para isso, clique com o botão direito do mouse sobre uma placa reconhecida para abrir a pesquisa
rápida com esta placa:
Com esta opção, o sistema irá trazer a tela de pesquisa, com os filtros de data e placa aplicados.
Você também poderá acessar a pesquisa rápida através do botão Pesquisa Rápida na interface do
objeto de LPR:
97
222 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Esta opção porém, irá exibir uma tela, para você preencher com a data e hora e as placas que deseja
pesquisar:
Preencha a data e hora de pesquisa e uma lista de placas (Uma por linha) e clique em OK. A tela
padrão de pesquisa de registros de LPR será exibida, com os filtros de data e placa já aplicados e a
pesquisa já iniciada:
LPR 223
© 2002 - 2024 por Digifort. Todos direitos reservados.
16.5.1.2 Detalhes do Registro
Os registros de LPR guardam no banco de dados a foto da placa capturada, assim como informações
extras. Para visualizar estas informações, você deverá abrir a guia de detalhes.
Para ver o detalhe de algum registro basta selecioná-lo e clicar em Detalhes do registro selecionado
como mostra a imagem abaixo:
O controle irá exibir a imagem da câmera principal e das câmeras periféricas, você poderá colocar o
mouse sobre uma imagem para ver ela maior:
Você poderá salvar uma imagem, para isso clique sobre a imagem desejada e um popup será exibido
com a opção para salvar a imagem em um diretório desejado.
O sistema permite a alteração de uma placa que não foi reconhecida corretamente, diretamente na tela
de pesquisa. Para isso, clique com o botão direito do mouse sobre um registro e selecione a opção
Modificar Placa:
A seguinte janela aparecerá, permitindo que o registro seja alterado pelo Administrador do sistema, ou
um operador com as permissões adequadas:
224 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
16.5.1.2.1 Exibir recorte da placa nos resultados
O visualizador de registros de LPR (Ao vivo e consulta) permite a exibição do recorte da placa reconhecida,
assim como a representação digital da placa reconhecida.
Nos registros ao vivo, o sistema permite escolher entre o recorte da imagem ou a representação virtual da
placa:
Nos detalhes de um registro na pesquisa de registros de LPR é possível configurar a exibição tanto do recorte
da placa quanto da placa virtual:
Estas opções podem ser alteradas na Configuração de LPR do Cliente de Monitoramento:
É possível também adicionar o recorte das placas nos relatórios impressos de LPR:
50
LPR 225
© 2002 - 2024 por Digifort. Todos direitos reservados.
16.5.1.3 Pesquisando com filtros
A pesquisa permite a seleção de diversos filtros para auxiliar na busca por registros. Clique no botão
Gerenciar Filtros para adicionar novos filtros:
16.5.1.3.1 Filtro de Data
O filtro de data permite pesquisar os registros pela data selecionada.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Data.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
Escolha intervalo de tempo para pesquisar os registros.
16.5.1.3.2 Filtro de Placas
O filtro de placa permite que uma placa ou diversas placas de interesses sejam localizadas nos
registros rapidamente.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Placas.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
16.5.1.3.2.1 Lidando com placas repetidas
No topo da tela de seleção de filtros, é possível configurar como o sistema irá lidar com placas repetidas
nesta pesquisa:
226 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
As opções são:
· Desativado: Caso essa opção seja selecionada, o sistema exibirá as placas repetidas normalmente,
em todas as suas ocorrências.
· Exibir apenas o primeiro registro da placa: Caso essa opção seja selecionada, o sistema exibirá
apenas a primeira captura da placa.
· Exibir apenas o último registro da placa: Caso essa opção seja selecionada, o sistema exibirá
apenas a captura mais recente da placa.
16.5.1.3.2.2 Pesquisa Simples
Na pesquisa simples a busca será feita pela placa inteira, ou seja, o que for digitado será procurado
como mostra a figura abaixo:
Após digitado a placa de interesse clique em OK.
Escolha intervalo de tempo para pesquisar os registros. Clique em OK e depois na tela principal de
pesquisa clique em Pesquisar:
16.5.1.3.2.3 Pesquisa Avançada
Na opção de filtro avançado, teremos uma gama maior de opções a para localizar um registro no banco
de dados. Selecione a opção Filtro Avançado e as seguintes opções estarão disponíveis:
LPR 227
© 2002 - 2024 por Digifort. Todos direitos reservados.
Essa tela possuí as seguintes funcionalidades:
As opções abaixo podem ser combinadas com lógicas E (AND) e OU (OR) com as condições Inicia
com, Termina com, Existe e Exato:
· Inicia com: Define o caractere ou caracteres a placa deve iniciar.
· Termina com: Define o caractere ou caracteres finais da placa.
· Existe: Define algum caractere ou combinação de caracteres existentes na placa na ordem desejada.
· Exato: Define a placa exata para a busca.
· E: Faz a lógica E com as combinações criando uma condição.
· OU: Faz a lógica OU com as combinações criando uma condição.
Por exemplo: Pesquisar placas que iniciem com "F" e terminem com "45":
16.5.1.3.3 Filtro de Velocidade
O filtro de velocidade permite pesquisar os registros de veículos capturados trafegando acima ou abaixo
de uma velocidade especificada.
Importante
O engine ou câmera deve suportar o recurso de detecção de velocidade para este filtro poder ser
aplicado.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Velocidade.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
· Velocidade Acima: Escolha esta opção para filtrar registros com velocidade acima do valor
especificado.
· Velocidade Abaixo: Escolha esta opção para filtrar registros com velocidade abaixo do valor
especificado.
16.5.1.3.4 Filtro de Câmeras
O filtro de câmera permite pesquisar os registros de placas reconhecidas nas câmeras especificadas.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Câmeras.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
228 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Selecione as câmeras desejadas, arrastando da lista esquerda para a lista da direita.
16.5.1.3.5 FIltro de Configurações de LPR
O filtro de Configuração de LPR permite pesquisar os registros de placas reconhecidas nas
Configurações de LPR especificadas.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Configurações de LPR.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
Selecione as Configurações de LPR desejadas, arrastando da lista esquerda para a lista da direita.
16.5.1.3.6 Filtro de Categoria
O filtro de categoria permite pesquisar os registros de placas reconhecidas de determinadas categorias.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Categorias.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
Selecione as categorias, arrastando da lista esquerda para a lista da direita.
16.5.1.3.7 Filtro de Classificação
O filtro de classificação permite pesquisar os registros de placas reconhecidas de classificações
especificadas.
Importante
O engine ou câmera deve suportar o recurso de classificação de placas para este filtro poder ser
aplicado.
LPR 229
© 2002 - 2024 por Digifort. Todos direitos reservados.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Classificação.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
Selecione as classificações, arrastando da lista esquerda para a lista da direita.
16.5.1.3.8 Filtro de Fabricante
O filtro de fabricantes permite pesquisar os registros de placas reconhecidas de veículos de fabricantes
específicos.
Importante
O engine ou câmera deve suportar o recurso de reconhecimento de fabricante de veículo para este filtro
poder ser aplicado.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Fabricantes.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
Selecione os fabricantes, arrastando da lista esquerda para a lista da direita.
16.5.1.3.9 Filtro de Modelo
O filtro de modelo permite pesquisar os registros de placas reconhecidas de veículos de modelos
específicos.
Importante
O engine ou câmera deve suportar o recurso de reconhecimento de modelo de veículo para este filtro
poder ser aplicado.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Modelos.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
230 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Selecione os modelos, arrastando da lista esquerda para a lista da direita.
16.5.1.3.10 Filtro de Cores
O filtro de cores permite pesquisar os registros de placas ou veículos de determinadas cores.
Importante
O engine ou câmera deve suportar o recurso de reconhecimento de cor de placa ou veículo para este
filtro poder ser aplicado.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Cores.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
O sistema poderá exibir filtro para cores de placas e também para cores de veículos, dependendo do
tipo de informação que está atualmente disponível no banco de dados.
Selecione as cores desejadas, arrastando da lista esquerda para a lista da direita.
16.5.1.3.11 Filtro de País
O filtro de país permite pesquisar os registros de placas reconhecidas de países específicos.
Importante
O engine ou câmera deve suportar o recurso de reconhecimento de país para este filtro poder ser
aplicado.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Países.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
LPR 231
© 2002 - 2024 por Digifort. Todos direitos reservados.
Selecione os países, arrastando da lista esquerda para a lista da direita.
16.5.1.3.12 Filtro de Lista
O filtro de lista permite pesquisar os registros de placas reconhecidas de que fazem parte de listas
específicas.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Listas.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
Selecione as listas, arrastando da lista esquerda para a lista da direita.
16.5.1.3.13 Filtro de Confiabilidade
O filtro de confiabilidade permite pesquisar os registros de placas que foram reconhecidas com
determinada confiabilidade.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Confiabilidade.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
232 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Selecione as confiabilidades, arrastando da lista esquerda para a lista da direita.
16.5.1.3.14 Filtro de Proprietário
O filtro de proprietário permite pesquisar os registros de placas de determinados proprietários.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Proprietários.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
Selecione os proprietários, arrastando da lista esquerda para a lista da direita.
16.5.1.3.15 Filtro de LPR Bridge
O filtro de LPR Bridge permite pesquisar os registros de placas de acordo com os dados associados,
recebidos pelo módulo de LPR Bridge.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba LPR Bridge.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
LPR 233
© 2002 - 2024 por Digifort. Todos direitos reservados.
· Exbir apenas registros com dados fornecidos pelo LPR Bridge: Selecione esta opção para que a
pesquisa possua apenas os registros que possuem algum dado recebido pelo LPR Bridge. Caso
nenhuma informação foi retornada pelo LPR Bridge para determinado registro de placa, este será
excluído do resultado.
16.5.1.3.16 Filtro de Servidor
O filtro de servidores permite pesquisar os registros de placas reconhecidas em servidores específicos.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Servidores.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
Selecione os servidores, arrastando da lista esquerda para a lista da direita.
16.5.1.4 Gerando Relatórios
A tela de pesquisa de registros de LPR permite que seja salvo ou impressos relatórios a partir dos
resultados da pesquisa atual.
Depois de pesquisar as placas de interesse clique sobre o botão Relatório.
234 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Um pop-up se abrirá com o titulo de configuração do relatório.
· Layout 1: Layout em formato de lista, com possibilidade de adicionar a foto da captura.
o Exibir imagem da câmera: Exibe o snapshot de captura no relatório
· Layout 2: Layout em formato de página única, onde cada captura será adicionada em uma página
completa do relatório, incluindo a imagem da captura e com opção para adicionar imagem das
câmeras periféricas.
o Exibir imagem das câmeras periféricas: Exibie o snapshot das câmeras periféricas no relatório.
LPR 235
© 2002 - 2024 por Digifort. Todos direitos reservados.
· Mostrar recorte da placa: Exibe o recorte da imagem da placa no relatório.
· Incluir Gráfico de Confiabilidade: Inclui o gráfico de confiabilidade no final do relatório.
· Incluir Gráfico de Acerto: Inclui o gráfico de acerto no final do relatório.
Selecione o tipo de agrupamento de registro. Você poderá agrupar os registros por diferentes opções.
Selecione as opções mais apropriadas para o seu relatório e clique em OK.
Agora selecione o formato e se você deseja visualizar apenas, imprimir ou exportar (*.pdf, or *..html) e
clique em OK e o visualizador padrão de relatórios será exibido:
16.5.1.5 Imprimindo um Registro
A pesquisa de registro de LPR permite a impressão de um único registro. Basta selecionar o ítem
desejado e clicar no botão Imprimir:
236 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Digite as Informações para a Impressão:
Agora selecione o formato e se você deseja visualizar apenas, imprimir ou exportar (*.pdf, or *..html) e
clique em OK e o visualizador padrão de relatórios será exibido:
O relatório de registro único será exibido:
LPR 237
© 2002 - 2024 por Digifort. Todos direitos reservados.
16.5.2 Gráfico de Confiabilidade
O LPR registra um nível de confiabilidade na leitura por caractere. O Software gera uma média e nos
mostra o grau de confiabilidade por placa.
Exemplo: A placa ABC1234 teve um índice de confibialidade no reconhecimento de 90 %, que é
considerado um indice alto de acerto.
Índices de confiabilidade:
· Alto: Confiabilidade maior igual que 90%, Letras da placa serão exibidas em preto:
· Médio: Confiabilidade entre 70% e 90%. Letras da placa serão exibidas em azul:
· Baixo: Confiabilidade menor que 70%: Letras da placa serão exibidas em vermelho:
Após pesquisar os registros das placas reconhecidas, é possível gerar um gráfico do índice de
confiabilidade. Para isso clique em Gráfico de Confiabilidade como mostra a imagem abaixo:
238 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
· Tipo de Gráfico: Selecione o tipo de gráfico.
o Barra: Gráfico de barras.
o Pizza: Gráfico de pizza.
· 3D: Adiciona efeito 3D ao gráfico.
· Legenda: Adiciona legenda aos valores do gráfico.
· Exibir Valores: Exibe os valores de contagem no gráfico.
· Imprimir: Imprime o gráfico atual.
Exemplo de gráfico em pizza:
O gráfico gerado pode ser impresso ou salvo clicando em Imprimir:
LPR 239
© 2002 - 2024 por Digifort. Todos direitos reservados.
16.5.3 Gráfico de Acertos
O sistema considera um acerto na leitura de placa quando o grau de confiabilidade, mencionado no
tópico anterior, é Alto ou Médio. Baseado nesta informação, o sistema possui um gráfico de acerto que
pode ser gerado a partir dos registros pesquisados.
Clique em Gráfico de Acertos para gerar um gráfico dessa informação como mostra a imagem abaixo:
· Tipo de Gráfico: Selecione o tipo de gráfico.
o Barra: Gráfico de barras.
o Pizza: Gráfico de pizza.
· 3D: Adiciona efeito 3D ao gráfico.
· Legenda: Adiciona legenda aos valores do gráfico.
· Exibir Valores: Exibe os valores de contagem no gráfico.
· Imprimir: Imprime o gráfico atual.
240 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
16.5.4 Traçar Rotas de Placas
O sistema de consulta de registros de LPR no Cliente de Monitoramento possui integração com Google
Maps, permitindo exibir todos os pontos de reconhecimento de uma placa no mapa, bastando apenas
cadastrar a posição de GPS das câmeras de LPR.
Primeiramente, as câmeras configuradas para gerar leitura de LPR, precisa ter suas coordenadas
configuradas. Consulte o Manual do Cliente de Administração para aprender a configurar as coordenadas:
Atela para traçar a rota de LPR irá exibir todos os pontos em que as placas informadas na pesquisa foram
encontradas no período especificado e irá ligar os pontos através do horário, e também é possível utilizar o
engine de rotas do Google Maps e traçar uma rota entre os pontos reconhecidos, porém para este recurso
funcionar corretamente é necessário que a placa tenha sido reconhecida em diversas câmeras para gerar
maior precisão da rota. Este é um excelente recurso para instalações de LPR em cidades inteligentes.
Os registros serão salvos no banco de dados do sistema, e os dados de Latitude e Longitude das
câmeras que geraram o registro, serão acompanhados pelos horários dos quais o veículo foi detectado,
a partir de tais dados, o Cliente de Monitoramento é capaz de traçar a rota do veículo em um mapa do
Google Maps.
LPR 241
© 2002 - 2024 por Digifort. Todos direitos reservados.
Clique na guia Traçar Rota de Placas:
· Placa: Digite o número da placa para ser localizada. Você poderá consultar múltiplas placas, para
isso, digite cada placa separada por vírgula.
· Data Inicial e Final: Horários e dias que devem ser pesquisados.
· Mostrar Rota: O sistema calcula, baseado nos horários e locais onde a placa foi detectada, a rota
percorrida pelo veículo, e a exibe na tela do mapa.
· Rotas Encontradas: Após a pesquisa, as placas localizadas serão adicionadas na coluna Rotas
Encontradas. Clique na placa desejada para ver a sua rota ou selecione Todos para ver todas as
rotas de todas as placas simultâneamente.
16.6 Zonas de LPR
As Zonas de LPR podem ser utilizadas para conceitos mais avançados como taxa de ocupação das
premissas monitoradas pelo sistema. Uma zona possui Configurações de LPR de Entrada e Saída.
Veículos reconhecidos pelas configurações associadas com a entrada da zona serão adicionados
dentro da zona, assim como veículos reconhecidos pelas configurações de saída da zona serão
removidos da zona.
O objeto de Zona de LPR no Cliente de Monitoramento irá exibir a quantidade de veículos dentro da
zona, assim como a quantidade de entradas e saídas do dia, taxa média de ocupação e a lista de todos
os veículos dentro da zona atualmente.
O sistema também permite a criação de Grupos de Zonas, com a função de agrupar, exibindo a
somatória dos registros de todas as zonas agrupadas.
O sistema ainda possui relatórios e pesquisas dos registros de entradas e saídas em zonas.
As Zonas de LPR podem ser encontrados na sua lista de objetos:
242 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Interface:
No lado esquerdo do controle, o sistema apresenta uma lista com as placas dentro da zona neste
momento.
No painel principal temos diversas informações:
· Dentro da zona: Quantas placas estão dentro dessa zona de LPR (passaram pela entrada mas
ainda não passaram pela saída).
· Entradas hoje: Quantas placas foram capturadas nas configurações de entrada no dia de hoje.
· Saídas hoje: Quantas saídas foram capturadas nas configurações de saída no dia de hoje.
· Tempo médio de ocupação: Qual é a média de tempo entre a entrada e saída dos veículos
capturados.
LPR 243
© 2002 - 2024 por Digifort. Todos direitos reservados.
O ítem indicando a quantidade de veículos dentro da zona, possui cores programadas (Através do
Cliente de Administração) para indicar se a capacidade da zona está sendo atingida:
16.6.1 Adicionando Placas Manualmente
Nos casos onde a câmera de entrada não reconheceu alguma placa, o operador poderá adicionar a
placa manualmente na zona.
Para adicionar uma placa na zona, clique com o botão direito no painel principal e selecione a opção
Adicionar nova placa na zona:
A tela para adicionar uma nova placa será exibida:
· Placa: Forneça a placa para ser adicionar na zona.
· Data de Entrada: Forneça a data e hora de entrada do veículo na zona. Este campo virá preenchido
automaticamente com a data e hora atual.
16.6.2 Removendo Placas Manualmente
Nos casos onde a câmera de saída não reconheceu alguma placa, o operador poderá remover a placa
manualmente na zona.
Para remover uma placa da zona, selecione o ítem desejado, na lista de placas, clique com o botão
direito do mouse sobre ele e selecione a opção Remover:
244 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
A tela para remover a placa será exibida:
· Remover e manter histórico: Remove a placa e adiciona um registro de saída da placa na zona
o Data de Saída: Forneça a data e hora de saída do veículo da zona. Este campo virá preenchido
automaticamente com a data e hora atual.
· Apenas remover: Remove a placa da zona, sem registrar a sua saída.
16.6.3 Pesquisa de Registros
O sistema permite uma consulta detalhada dos registros de entrada e saída das zonas de LPR.
Para abrir a tela de consulta, clique sobre o botão Zonas de LPR, no Menu de Opções:
A seguinte tela será exibida:
Por padrão, o filtro de data com o dia atual será aplicado, e ao clicar em pesquisar, os eventos serão
exibidos como mostra a imagem abaixo:
LPR 245
© 2002 - 2024 por Digifort. Todos direitos reservados.
Você poderá reproduzir o vídeo da entrada e da saída do veículo da zona. Para isso, selecione o registro
desejado e clique em Vídeo de Entrada para visualizar o vídeo da entrada do veículo na zona e Vídeo
de Saída para visualizar o vídeo de saída do veículo da zona. O Reprodutor de Mídia será aberto.
16.6.3.1 Filtros
A pesquisa permite a seleção de diversos filtros para auxiliar na busca por registros. Clique no botão
Gerenciar Filtros para adicionar novos filtros:
16.6.3.1.1 Filtro de Data
O filtro de data permite pesquisar os registros pela data selecionada.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Data.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
· Data de Entrada: Ative para filtrar os registros pela sua data de entrada.
o Data Inicial e Final: Escopo de data e hora para filtro pela data de entrada.
· Data de Saída: Ative para filtrar os registros pela sua data de saída.
o Data Inicial e Final: Escopo de data e hora para filtro pela data de saída.
· Apenas exibir registros de placas que estão na zona: Filtra os resultados para exibir apenas os
registros de placa que ainda estão dentro da zona.
97
246 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
16.6.3.1.2 Filtro de Placa
O filtro de placa permite pesquisar os registros de entrada e saída de zonas de LPR de placas
específicas.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Placas.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
Digite as placas desejadas para filtrar. Forneça uma placa por linha.
16.6.3.1.3 Filtro de Zona
O filtro de zona permite pesquisar os registros de entrada e saída de uma zona específica.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Zonas.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
Selecione os ítens desejados, arrastando da lista esquerda para a lista da direita.
16.6.3.1.4 Filtro de Grupo de Zona
O filtro de grupo de zona permite pesquisar os registros de entrada e saída de uma zonas que fazem
parte de grupos específicos.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Zonas.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
Selecione os ítens desejados, arrastando da lista esquerda para a lista da direita.
16.6.3.1.5 Filtro de Servidores
O filtro de servidores permite pesquisar os registros de zonas de LPR de servidores específicos.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Servidores.
LPR 247
© 2002 - 2024 por Digifort. Todos direitos reservados.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
Selecione os servidores, arrastando da lista esquerda para a lista da direita.
16.6.4 Impressão de Registros
Clicando no botão de Relatório é possível gerar um relatório para impressão com todos os registros
filtrados:
Selecione o tipo de agrupamento de registro. Você poderá agrupar os registros por diferentes opções.
Selecione as opções mais apropriadas para o seu relatório e clique em OK.
Agora selecione o formato e se você deseja visualizar apenas, imprimir ou exportar (*.pdf, or *..html) e
clique em OK e o visualizador padrão de relatórios será exibido:
248 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
O relatório gerado será parecido com a imagem abaixo:
Chapter
XVII
250 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
17 Logs de Eventos
O log de eventos permite que qualquer evento do sistema possa ser encontrado rapidamente,
visualizado e utilizado como dado para a recuperação de uma gravação.
Para abrir a tela de eventos, clique sobre o botão Log de eventos no Menu de Opções:
A seguinte tela será exibida:
17.1 Detalhes do Registro
Todo registro gravado possui detalhes extendidos que podem ser visualizados através do painel de
detalhes.
Clique no botao Detalhes do Registro Selecionado para abrir a tela de detalhes. Você poderá manter
este painel aberto enquanto navega entre diferentes registros.
Logs de Eventos 251
© 2002 - 2024 por Digifort. Todos direitos reservados.
No painel esquerdo dos detalhes, será exibido informações sobre o evento, como o tipo de evento, data
e hora de disparo e detalhes adicionais. Cada tipo de evento poderá ter detalhes diferentes, pertinentes
ao seu tipo.
O painel direito, será exibido informações sobre as ações geradas a partir deste evento, como por
exemplo: e-mails enviados, câmeras que foram mostradas na tela em pop-up, mensagens enviadas,
resposta dos operados a um alarme, etc.
Caso o evento possua coordenadas geográficas associadas, você poderá visualizar o local de disparo do
evento em um mini mapa, que pode ser exibido ao clicar no botão Exibir Evento no Mapa.
No canto inferior direito ainda poderá ser acionado o botão Vídeo. Ao clicar, ele abrirá o Reprodutor de
Mídia, com o vídeo do horário em que o evento ocorreu, e com as câmeras associadas nas ações de
alarmes do evento.
17.2 Filtros
A pesquisa permite a seleção de diversos filtros para auxiliar na busca por registros. Clique no botão
Gerenciar Filtros para adicionar novos filtros:
17.2.1 Filtro de Data
O filtro de data permite pesquisar os registros pela data selecionada.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Data.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
Escolha intervalo de tempo para pesquisar os registros.
17.2.2 Filtro de Entrada de Alarme
O filtro de entrada de alarme permite pesquisar os registros de eventos disparados a partir de entradas
de alarme (I/O) de câmeras ou dispositivos de I/O.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Entrada de Alarme.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
252 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Você poderá filtrar eventos por dispositivos de I/O, câmeras ou por eventos específicos. Selecione os
ítens desejados, arrastando da lista esquerda para a lista da direita.
17.2.3 Filtro de Comunicação
O filtro de comunicação permite pesquisar os registros de eventos disparados quando dispositivos ou
objetos do sistema ficam fora de funcionamento ou retornam para o estado de funcionamento.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Comunicação.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
Diferentes tipos de objetos podem disparar os eventos de Falha e Restauração de Comunicacão e você
poderá selecionar os objetos que deseja filtrar, assim como também selecionar o tipo de evento de
comunicação, sendo Falha ou Restauração da comunicaçao. Selecione os ítens desejados,
arrastando da lista esquerda para a lista da direita.
17.2.4 Filtro de Gravação
O filtro de gravação permite pesquisar os registros de eventos disparados quando ocorrem erros de
gravação em câmeras.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Gravação.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
Logs de Eventos 253
© 2002 - 2024 por Digifort. Todos direitos reservados.
Você poderá selecionar os dispositivos que deseja filtrar, assim como também selecionar o tipo de
evento de gravação, sendo Falha ou Restauração da gravação. Selecione os ítens desejados,
arrastando da lista esquerda para a lista da direita.
17.2.5 Filtro de Deteção de Movimento
O filtro de detecção de movimento permite pesquisar os registros de eventos de detecção de movimento
disparados por câmeras.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Detecção de Movimento.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
Selecione os ítens desejados, arrastando da lista esquerda para a lista da direita.
17.2.6 Filtro de Detecção de Nível de Áudio
O filtro de detecção de nível de áudio permite pesquisar os registros de eventos disparados quando
câmeras reconhecem que o nível de áudio está muito alto ou muito baixo.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Detecçao de Nível de
Áudio.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
Você poderá selecionar os dispositivos que deseja filtrar, assim como também selecionar o tipo de
evento de deteção, sendo Nível Alto ou Nível Baixo. Selecione os ítens desejados, arrastando da lista
esquerda para a lista da direita.
17.2.7 Filtro de Evento Manual
O filtro de evento manual permite pesquisar os registros de eventos manuais disparados por operadores
do sistema.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Evento Manual.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
254 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Você poderá selecionar dentro diversos tipos de filtros como Dispositivo (Onde o evento manual está
cadastrado), Evento, IP da estação que disparou o evento ou Usuário que disparou o evento. Selecione
os ítens desejados, arrastando da lista esquerda para a lista da direita.
17.2.8 Filtro de Evento Programado
O filtro de evento programado permite pesquisar os registros de eventos disparados por eventos
programados.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Eventos Programados.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
Selecione os ítens desejados, arrastando da lista esquerda para a lista da direita.
17.2.9 Filtro de Evento Global
O filtro de evento global permite pesquisar os registros de eventos globais disparados por operadores do
sistema.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Eventos Globais.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
Logs de Eventos 255
© 2002 - 2024 por Digifort. Todos direitos reservados.
Esta tela permite filtrar os eventos por:
· Evento: Filtro por nome do evento disparado.
· User: Filtro por operador que disparou o evento.
· IP: Filtro por IP da estação utilizada para disparar o evento.
Selecione os ítens desejados, arrastando da lista esquerda para a lista da direita.
Adicionalmente, é possível adicionar filtros para as mensagens do evento (Caso disponível).
Condições:
· Começa com: O sistema filtrará por mensagens que comecem com o texto digitado.
· Termina com: O sistema filtrará por mensagens que terminem com o texto digitado.
· Contém: O sistema filtrará por mensagens que contenham o texto digitado em qualquer ponto da
mensagem.
· Exato: O sistema filtrará por mensagens que contenham exatamente o texto digitado.
Lógica:
· E: O sistema considerará apenas mensagens que possuam todos os textos adicionados.
· OU: O sistema considerará todas as mensagens que contenham pelo menos um dos textos
adicionados.
Selecione a condição, a lógica e o texto desejado para filtrar e clique em Adicionar.
17.2.10 Filtro de Evento de Dispositivo
Alguns dispositivos do sistema possuem eventos extras, chamados Evento de Dispositivo. O filtro de
evento de dispositivo permite pesquisar os registros destes eventos.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Evento de Dispositivo.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
256 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Você poderá selecionar os dispositivos que deseja filtrar, assim como também selecionar o tipo de
evento de evento de dispositivo. Selecione os ítens desejados, arrastando da lista esquerda para a lista
da direita.
17.2.11 Filtro de Evento de Analítico
O filtro de evento de analítico permite pesquisar os registros de eventos de sistema gerados a partir de
configurações de analíticos.
Apesar de similares, os Eventos de Analítico são diferentes dos Registros de Analítico . A diferença
entre esses dois módulos é fundamentalmente que os Eventos são os sub-produtos de um Registro de
Analítico. Os Registros de Analítico são gerados a partir do engine e possuem muitas informações de
metadados associadas ao registro, porém os registros de analítico não disparam ações, para isso, eles
são convertidos em um Evento de Sistema, com informações reduzidas, e assim podem ser utilizados
no sistema para disparar ações, assim como serem utilizados como entrada para outros eventos.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Analíticos.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
Você poderá selecionar os tipos de regra que deseja filtrar, assim como também selecionar as câmeras
e zonas de analítico. Selecione os ítens desejados, arrastando da lista esquerda para a lista da direita.
17.2.12 Filtro de Evento de LPR
O filtro de evento de LPR permite pesquisar os registros de eventos de LPR (Cadastrados no Cliente de
Administração).
Os Eventos de LPR são diferentes dos Registros de LPR . Os Registros de LPR possuem todas as
placas reconhecidas pelo sistema, enquan to os Eventos de LPR são apenas os eventos disparados de
acordo com alguma condição, como por exemplo quando um veículo reconhecido faz parte de uma lista
negra.
185
220
Logs de Eventos 257
© 2002 - 2024 por Digifort. Todos direitos reservados.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Reconhecimento de
Placas.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
Você poderá selecionar as câmeras e eventos. Selecione os ítens desejados, arrastando da lista
esquerda para a lista da direita.
As placas reconhecidas não serão exibidas na lista de filtro pois esta lista poderia ser muito extensa,
para filtrar por uma placa específica, digite a placa no campo Adicionar Placa e clique no botão
adicionar.
17.2.13 Filtro de Evento de Zona de LPR
O filtro de evento de zona de LPR permite pesquisar os registros de eventos disparados por zonas de
LPR.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Zona de LPR.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
258 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Você poderá selecionar as zonas e tipo de evento desejado. Selecione os ítens desejados, arrastando
da lista esquerda para a lista da direita.
17.2.14 Filtro de Evento de Servidor
O filtro de evento de servidor permite pesquisar os registros de eventos de saúde de servidor.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Eventos de Servidor.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
Selecione os tipos de eventos desejados, arrastando da lista esquerda para a lista da direita.
17.2.15 Filtro de Servidores
O filtro de servidores permite pesquisar os registros de eventos disparados em servidores específicos.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Servidores.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
Selecione os servidores, arrastando da lista esquerda para a lista da direita.
Logs de Eventos 259
© 2002 - 2024 por Digifort. Todos direitos reservados.
17.3 Imprimindo um Registro
A pesquisa de registros de eventos permite a impressão dos registros pesquisados. Para isso, clique no
botão Imprimir, após realizar a pesquisa.
Agora selecione o formato e se você deseja visualizar apenas, imprimir ou exportar (*.pdf, or *..html) e
clique em OK e o visualizador padrão de relatórios será exibido:
O relatório será exibido:
260 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
17.4 Relatórios
O sistema fornece alguns relatórios relacionados aos eventos. Clique na aba Relatórios:
Clique no relatório desejado. Verifique nos capítulos a seguir o detalhamento de cada tipo de relatório.
Todos os relatórios podem ser exportados para os formatos: PDF, CSV, TXT, RTF, XLS e HTML.
A tela permite que a logomarca seja alterada afim de personalizar o relatório. Basta clicar em Modificar
e escolher outro arquivo de imagem.
17.4.1 Relatório de Respostas dos Operadores aos Eventos
O relatório de Respostas dos Operadores aos Eventos contempla as informações digitadas pelos
operadores nos pop-ups de alarme, assim, você poderá verificar as ações tomadas pelos operadores
para todos os eventos disparados.
Clique no botão Respostas dos Operadores aos Eventos e a tela de Filtros será exibida. Você
deverá fornecer os filtros desejados (Este relatório é independente da pesquisa de registros).
Após configado os filtros, o sistema irá começar a gerar o relatório:
251
Logs de Eventos 261
© 2002 - 2024 por Digifort. Todos direitos reservados.
Após finalizado a pesquisa, o sistema irá disponibilizar o botão para abrir o relatório. Clique em Abrir
Relatório para visualizar, imprimir ou exportar.
17.4.2 Relatório de Falha de Dispositivos
O relatório de falha de dispositivos irá listar todas as falhas e recuperação de comunicação com os
dispositivos do sistema, fornecendo também o tempo total de falha de cada dispositivo.
Clique no botão Falhas de Dispositivos e a tela de Filtros será exibida com opções reduzidas.
Você deverá fornecer os filtros desejados (Este relatório é independente da pesquisa de registros).
Após configado os filtros, o sistema irá começar a gerar o relatório:
Após finalizado a pesquisa, o sistema irá disponibilizar o botão para abrir o relatório. Clique em Abrir
Relatório para visualizar, imprimir ou exportar.
251
262 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Chapter
XVIII
264 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
18 Logs de Auditoria
O recurso de Auditoria tem por objetivo registrar todas as ações dos usuários usuários no sistema e
conexões ao servidor.
Para abrir a tela de auditoria, clique sobre o botão Auditoria, no Menu de Opções:
A seguinte tela será exibida:
Por padrão, o filtro de data com o dia atual será aplicado, e ao clicar em pesquisar, os eventos serão
exibidos como mostra a imagem abaixo:
18.1 Filtros
A pesquisa permite a seleção de diversos filtros para auxiliar na busca por registros. Clique no botão
Gerenciar Filtros para adicionar novos filtros:
Logs de Auditoria 265
© 2002 - 2024 por Digifort. Todos direitos reservados.
18.1.1 Filtro de Data
O filtro de data permite pesquisar os registros pela data selecionada.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Data.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
Escolha intervalo de tempo para pesquisar os registros.
18.1.2 Filtro de Categoria
O filtro de categoria permite pesquisar os registros de auditoria de acordo com a sua categoria
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Categoria.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
· Conexões com o Servidor: Esta categoria registra todas as conexões de usuários com o servidor.
· Ações de Usuários: Esta categoria registra todas as ações de usuários no sistema, como alteração
de parâmetros e visualização de câmeras.
Selecione os ítens desejados, arrastando da lista esquerda para a lista da direita.
18.1.3 Filtro de Tipo de Evento
O filtro de tipo de evento permite pesquisar os registros de auditoria de acordo com o seu tipo.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Tipo de Evento.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
266 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
· Advertências: Este tipo de evento registra todas as ações de usuários relativas à administração do
sistema, como por exemplo adicionar/remover objetos, alterar cadastros de câmeras ou usuários, etc.
· Erros: Este tipo de evento registra erros, como por exemplo uma conexão de usuário recusada por
falha de autenticação.
· Informações: Este tipo de evento registra logs de carater informativo, como visualização de câmeras.
Selecione os ítens desejados, arrastando da lista esquerda para a lista da direita.
18.1.4 Filtro de Evento
O filtro de evento permite pesquisar os registros de auditoria para determinados eventos.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Evento.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
A lista será populada de acordo com os eventos disponíveis. O sistema possui muitos tipos de eventos,
você deverá selecionar os eventos de acordo com a sua necessidade. Se você deseja por exemplo
procurar por alterações em objetos (Quando um usuário altera um objeto), selecione o evento
Modificado. Se deseja procurar por objetos adicionados (Quando um usuário cria um novo objeto no
sistema), selecione o evento Adicionado.
Selecione os ítens desejados, arrastando da lista esquerda para a lista da direita.
18.1.5 Filtro de Tipo de Objeto
O filtro de tipo de objeto permite pesquisar os registros de auditoria de acordo com tipo de objeto
afetado.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Tipo de Objeto.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
Logs de Auditoria 267
© 2002 - 2024 por Digifort. Todos direitos reservados.
Selecione os ítens desejados, arrastando da lista esquerda para a lista da direita.
18.1.6 Filtro de Palavra-Chave
O filtro de palavra-chave permite pesquisar os registros por um texto ou palavra. Este texto será
consultado nos campos de Usuário, IP, Nome do Objeto e Complemento.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Palavra-Chave.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
· Palavra-Chave: Forneça o texto ou palavra para pesquisar.
· Procurar por palavra-chave exata: Com esta opção ativada, o texto deverá corresponder
exatamente ao que está gravado no campo. Se esta opção estiver desativada, o campo será
pesquisado para verificar se há uma ocorrência do texto, não sendo necessário a correspondência
completa. OBS: Desativar esta opção irá fazer com que a pesquisa seja mais lenta.
18.1.7 Filtro de Servidores
O filtro de servidores permite pesquisar os registros de auditoria registrados em servidores específicos.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Servidores.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
268 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Selecione os servidores, arrastando da lista esquerda para a lista da direita.
18.2 Detalhes de Registros
A auditoria de ações de usuário possui detalhes sobre alterações no sistema. Qualquer alteração feita
sobre qualquer objeto será registrada nos detalhes do registro de auditoria. Por exemplo, se um usuário
alterar o diretório de gravação de uma câmera, ou o tempo de gravação configurado, estas informações
serão detalhadas no registro de auditoria, incluindo os valores antigos e os valores novos.
A auditoria detalhada é aplicada para todos os objetos do sistema, incluindo configurações do servidor,
tornando assim a ferramenta de auditoria super poderosa para registrar e identificar ações de usuário no
sistema.
Campos críticos como senha ou campos binários (ou containers de armazenamento de dados) que não
podem ser exibidos em texto serão apenas referenciados como "alterado" mas seus valores não serão
exibidos.
Para acessar os detalhes de um registros, clique duas vezes com o botão esquedo do mouse sobre o
registro desejado e a tela de detalhes será exibida:
· Data do Registro: Data deste registro.
· Usuário: Usuário que efetuou a ação.
· IP: IP da estação utilizada pelo usuário para efetuar esta ação.
· Evento: Ação que usuário tomou.
· Tipo de Objeto: Tipo de objeto afetado pela ação.
Logs de Auditoria 269
© 2002 - 2024 por Digifort. Todos direitos reservados.
· Nome do Objeto: Nome do objeto afetado.
· Categoria: Categoria desta ação.
· Complemento: Contém dados complementares, como quais alterações foram feitas em um objeto.
18.3 Impressão de Registros
Clicando no botão de Relatório é possível gerar um relatório para impressão com todos os registros
filtrados:
Selecione o tipo de agrupamento de registro. Você poderá agrupar os registros por diferentes opções.
Selecione as opções mais apropriadas para o seu relatório e clique em OK.
Agora selecione o formato e se você deseja visualizar apenas, imprimir ou exportar (*.pdf, or *..html) e
clique em OK e o visualizador padrão de relatórios será exibido:
O relatório gerado será parecido com a imagem abaixo:
270 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Chapter
XIX
272 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
19 Proteção de Gravações
O sistema permite criar bloqueios contra a exclusão natural de gravações, ou seja, as gravações
protegidas de um período e câmeras especificadas não serão excluídas durante a reciclagem de
gravações enquanto o bloqueio existir.
Para maior proteção, o sistema de bloqueio de gravações ao invés de impedir que um arquivo de
gravação seja apagado do disco principal, irá copiar os arquivos protegidos para uma outra pasta segura
(que pode estar em outra unidade de disco) configurada na aba Gravações nas opções de sistema no
Cliente de Administração. A cópia do arquivo ocorre somente durante o processo de reciclagem, ou
seja, ao invés de apagar o arquivo, o sistema irá mover o arquivo para a pasta protegida, o que impede a
duplicação das gravações pois a cópia só irá ocorrer se o arquivo protegido for o mais antigo no disco.
Isto permite que os discos primários de gravações não sejam ocupados por gravações protegidas que
excedem o período normal de retenção, liberando espaço para novas gravações, enquanto ainda fornece
a proteção das gravações.
A reprodução dos vídeos bloqueados (após serem movidos para a nova pasta) será totalmente
transparente para o usuário.
Nota
Os arquivos de gravação do sistema são gerados a cada 30 minutos de vídeo, se um pequeno bloco de
tempo de por exemplo 2 minutos for bloqueado, todo o arquivo de 30 minutos será protegido. Caso o
bloqueio se estenda por múltiplos arquivos, todos os arquivos serão bloqueados.
19.1 Protegendo uma Gravação
Para criar uma nova proteção de gravações basta criar um novo bookmark com o período desejado e
selecionar a opção Proteger gravações contra exclusão e um novo registro de proteção será criado
para as câmeras especificadas no bookmark e o período desejado.
· Proteger gravações contra deleção: Selecione esta opção para proteger as gravações do período
deste bookmark.
o Adicionar data de expiração: Selecione esta opção para que a proteção destas gravações expire
em um dia configurado.
§ Data: Selecione a data de expiração da proteção.
Por padrão, o bloqueio das gravações será perpétuo, a menos que a opção Adicionar data de
expiração for selecionada, nesse caso o bloqueio será excluído na data especificada e
consequentemente as gravações entrarão novamente na reciclagem de gravações e serão apagadas
normalmente durante a manutenção dos vídeos.
143
Proteção de Gravações 273
© 2002 - 2024 por Digifort. Todos direitos reservados.
19.2 Consultando Gravações Protegidas
Para abrir a tela de consulta de gravações protegidas, clique sobre o botão Gravações Protegidas, no
Menu de Opções:
A seguinte tela será exibida:
Por padrão, o sistema não irá aplicar nenhum filtro. Clique no botão Pesquisar para exibir todas as
gravações protegidas.
Clique no botão Vídeo para reproduzir o vídeo protegido.
274 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
19.2.1 Filtros
A pesquisa permite a seleção de diversos filtros para auxiliar na busca por registros. Clique no botão
Gerenciar Filtros para adicionar novos filtros:
19.2.1.1 Filtro de Data
O filtro de data permite pesquisar os registros pela data selecionada.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Data.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
Escolha intervalo de tempo para pesquisar os registros.
19.2.1.2 Filtro de Expiração
O filtro de expiração permite pesquisar os registros pela sua data de expiração.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Expiração.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
· Exibir todos os registros protegidos: Aplica o filtro no campo de expiração, com o escopo de data e
hora fornecida, para todos os registros.
· Exibir apenas registros com data de expiração: Aplica o filtro no campo de expiração, com o
escopo de data e hora fornecida, apenas para os registros que possuem data de expiração.
· Exibir apenas registros sem data de expiração: Filtra e exibe apenas os registros que não
possuem data de expiração. Nesta opção, não é possível fornecer o escopo de data e hora.
· Data e Hora Inicial e Final: Escopo de data e hora inicial e final para filtro de data de expiração.
19.2.1.3 Filtro de Câmeras
O filtro de câmera permite pesquisar os registros de gravações protegidas nas câmeras especificadas.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Câmeras.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
Proteção de Gravações 275
© 2002 - 2024 por Digifort. Todos direitos reservados.
Selecione as câmeras desejadas, arrastando da lista esquerda para a lista da direita.
19.2.1.4 Filtro de Usuários
O filtro de câmera permite pesquisar os registros de gravações protegidas criados por usuários
selecionados.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Usuários.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
Selecione os usuários desejadas, arrastando da lista esquerda para a lista da direita.
19.2.1.5 Filtro de Servidores
O filtro de servidores permite pesquisar os registros de gravações protegidas em servidores específicos.
Para adicionar o filtro clique em Gerenciar Filtros, e depois clique na aba Servidores.
Para ativar o filtro basta clicar no checkbox ao lado direito do filtro.
Selecione os servidores, arrastando da lista esquerda para a lista da direita.
19.3 Impressão de Registros
Clicando no botão de Relatório é possível gerar um relatório para impressão com todos os registros
filtrados:
276 Cliente de Monitoramento - Professional 7.4.1
© 2002 - 2024 por Digifort. Todos direitos reservados.
Selecione o tipo de agrupamento de registro. Você poderá agrupar os registros por diferentes opções.
Selecione as opções mais apropriadas para o seu relatório e clique em OK.
Agora selecione o formato e se você deseja visualizar apenas, imprimir ou exportar (*.pdf, or *..html) e
clique em OK e o visualizador padrão de relatórios será exibido:
O relatório gerado será parecido com a imagem abaixo:
Proteção de Gravações 277
© 2002 - 2024 por Digifort. Todos direitos reservados.
Chapter
XX
Alterando a senha de usuário 279
© 2002 - 2024 por Digifort. Todos direitos reservados.
20 Alterando a senha de usuário
O Cliente de Monitoramento dispõe da funcionalidade de troca de senha do usuário nos servidores
logados. Para isso pressione a tecla F12 do seu teclado, exibindo a tela de troca de senha, conforme
ilustrado na figura abaixo.
Nesta tela são listados todos os servidores em que você está conectado e os seus respectivos
usuários.
Para trocar a senha de algum usuário, dê um duplo clique no servidor desejado, abrindo a tela abaixo:
Digite a sua senha atual, a nova senha e a confirmação da nova senha.
Se todos os dados estiverem corretos a senha será alterada e deverá ser utilizada no próximo login.
Se o servidor com a senha alterada estiver com a opção auto login habilitada, será necessária a
alteração desta configuração, digitando a nova senha.
Você só poderá trocar a senha do usuário se ele for um usuário nativo do sistema. Você não poderá
trocar a senha de um usuário Active Directory.