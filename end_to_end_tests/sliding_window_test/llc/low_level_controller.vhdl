library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.fixed_pkg.all;
use work.array_type_pkg.all;

entity low_level_controller is
    port (
        clk, eclk, rst : in std_logic;
        time_in : in unsigned(63 downto 0);
		a : in signed(31 downto 0);
		a_en : in std_logic;
		s_s_en : in std_logic;
		c_s_en : in std_logic;
		av_s_en : in std_logic;
		a_u_en : in std_logic;
		s_u_en : in std_logic;
		c_u_en : in std_logic;
		av_u_en : in std_logic;
		data_available : in std_logic;
		s_s : out signed(31 downto 0);
		c_s : out unsigned(63 downto 0);
		av_s : out signed(31 downto 0);
		a_u : out unsigned(31 downto 0);
		s_u : out unsigned(31 downto 0);
		c_u : out unsigned(63 downto 0);
		av_u : out unsigned(31 downto 0);
		pop : out std_logic;
		eval_done : out std_logic
    );
end low_level_controller;

architecture mixed of low_level_controller is

	-- component declaration
	component evaluator is
		port (
			clk, input_clk, rst : in std_logic;
			input_time : in unsigned(63 downto 0);
			a : in signed(31 downto 0);
			a_en : in std_logic;
			s_s_en : in std_logic;
			c_s_en : in std_logic;
			av_s_en : in std_logic;
			a_u_en : in std_logic;
			s_u_en : in std_logic;
			c_u_en : in std_logic;
			av_u_en : in std_logic;
			s_s : out signed(31 downto 0);
			c_s : out unsigned(63 downto 0);
			av_s : out signed(31 downto 0);
			a_u : out unsigned(31 downto 0);
			s_u : out unsigned(31 downto 0);
			c_u : out unsigned(63 downto 0);
			av_u : out unsigned(31 downto 0);
			done : out std_logic;
			valid : out std_logic
		);
	end component;

	-- signal declaration
	signal input_clk : std_logic;
	signal current_state : integer;
	signal evaluator_done : std_logic;
	signal evaluator_valid : std_logic;
	signal pop_data : std_logic;

begin
    -- component instantiation
    evaluator_instance: evaluator
        port map (
			clk => clk,
			input_clk => input_clk,
			rst => rst,
			input_time => time_in,
			a => a,
			a_en => a_en,
			s_s_en => s_s_en,
			c_s_en => c_s_en,
			av_s_en => av_s_en,
			a_u_en => a_u_en,
			s_u_en => s_u_en,
			c_u_en => c_u_en,
			av_u_en => av_u_en,
			s_s => s_s,
			c_s => c_s,
			av_s => av_s,
			a_u => a_u,
			s_u => s_u,
			c_u => c_u,
			av_u => av_u,
			done => evaluator_done,
			valid => evaluator_valid
        );

    process(eclk, rst) begin
		if rst='1' then
			input_clk <= '0';
			current_state <= 0;
			pop_data <= '0';
		elsif rising_edge(eclk) then
            if (current_state = 0 and data_available = '1') then
                -- idle
                pop_data <= '1';
                input_clk <= '0';
                current_state <= 1;
            elsif current_state = 1 then
                -- pop
                input_clk <= '1';
                pop_data <= '0';
                current_state <= 2;
            elsif current_state = 2 and evaluator_done = '1' then
                -- evaluate_done
                if data_available = '1' then
                    pop_data <= '1';
                    input_clk <= '0';
                    current_state <= 1;
                else
                    input_clk <= '0';
                    current_state <= 0;
                end if;
            end if;
        end if;
	end process;

	pop <= pop_data;
	eval_done <= input_clk;

end mixed;