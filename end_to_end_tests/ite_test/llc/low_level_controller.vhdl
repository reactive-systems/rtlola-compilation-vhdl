library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.fixed_pkg.all;
use work.array_type_pkg.all;

entity low_level_controller is
    port (
        clk, eclk, rst : in std_logic;
        time_in : in unsigned(63 downto 0);
		a : in signed(7 downto 0);
		a_en : in std_logic;
		b : in signed(15 downto 0);
		b_en : in std_logic;
		val : in std_logic;
		val_en : in std_logic;
		c_en : in std_logic;
		d_en : in std_logic;
		e_en : in std_logic;
		counter_en : in std_logic;
		data_available : in std_logic;
		c : out unsigned(7 downto 0);
		d : out signed(15 downto 0);
		e : out signed(7 downto 0);
		counter : out signed(63 downto 0);
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
			a : in signed(7 downto 0);
			a_en : in std_logic;
			b : in signed(15 downto 0);
			b_en : in std_logic;
			val : in std_logic;
			val_en : in std_logic;
			c_en : in std_logic;
			d_en : in std_logic;
			e_en : in std_logic;
			counter_en : in std_logic;
			c : out unsigned(7 downto 0);
			d : out signed(15 downto 0);
			e : out signed(7 downto 0);
			counter : out signed(63 downto 0);
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
			b => b,
			b_en => b_en,
			val => val,
			val_en => val_en,
			c_en => c_en,
			d_en => d_en,
			e_en => e_en,
			counter_en => counter_en,
			c => c,
			d => d,
			e => e,
			counter => counter,
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